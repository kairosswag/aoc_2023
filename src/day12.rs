use crate::day12::Spring::{Damaged, Unknown};
use itertools::Itertools;
use std::fs;
use std::ops::Add;
use std::str::Lines;
use std::time::Instant;

pub fn run() {
    let file = fs::read_to_string("input/day12").expect("Could not open file.");
    let now = Instant::now();
    let (res_1, res_2) = solve(file.lines());
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 12 Solution Part 1: {}", res_1);
    println!("Day 12 Solution Part 2: {}", res_2);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug)]
struct Row {
    row: Vec<Spring>,
    damage_groups: Vec<usize>,
}

impl Row {
    fn calculate_arrangements(&self) -> usize {
        Self::sub_calculate(&self.row, &self.damage_groups, String::from(""))
    }

    fn sub_calculate(rest_row: &[Spring], rest_groups: &[usize], curr: String) -> usize {
        let mut curr = curr;
        if rest_groups.len() == 0 {
            if rest_row.iter().any(|filter| *filter == Damaged) {
                return 0;
            }
            println!(
                "Finishing {}{}",
                curr,
                rest_row
                    .iter()
                    .map(|val| match val {
                        Damaged => "WTF",
                        Spring::Operational => ".",
                        Spring::Unknown => "?",
                    })
                    .collect::<String>()
            );
            return 1;
        } else if rest_row.len() < rest_groups[0] {
            return 0;
        }
        let curr_group = rest_groups[0];
        let mut total = 0;
        'outer: for i in 0..=rest_row.len() - curr_group {
            let all_following_may_be_damaged = rest_row[i..i + curr_group]
                .iter()
                .all(|val| *val == Damaged || *val == Unknown);
            match (all_following_may_be_damaged, rest_row.get(i + curr_group)) {
                (true, Some(Unknown)) | (true, Some(Spring::Operational)) => {
                    let res = Self::sub_calculate(
                        &rest_row[i + curr_group + 1..],
                        &rest_groups[1..],
                        Self::add_string(&curr, curr_group, false),
                    );
                    if res == 0 {
                        break 'outer;
                    } else {
                        total += res;
                        if rest_row[i] == Damaged {
                            break 'outer;
                        }
                    }
                }
                (true, None) => {
                    let res = Self::sub_calculate(
                        &rest_row[i + curr_group..],
                        &rest_groups[1..],
                        Self::add_string(&curr, curr_group, true),
                    );
                    if res == 0 {
                        break 'outer;
                    } else {
                        total += res;
                        if rest_row[i] == Damaged {
                            break 'outer;
                        }
                    }
                }
                (false, _) | (true, Some(Damaged)) => {
                    if rest_row[i] == Damaged {
                        break 'outer;
                    }
                }
            }
            curr += match rest_row[i] {
                Damaged => "Ä",
                Spring::Operational => ".",
                Spring::Unknown => "?",
            };
        }
        total
    }

    fn add_string(curr: &str, curr_group: usize, last: bool) -> String {
        let res = String::from(curr) + &"#".repeat(curr_group);
        if last {
            res
        } else {
            res + &"."
        }
    }
}

fn solve(lines: Lines) -> (usize, usize) {
    (
        lines
            .map(|line| {
                line.split_whitespace()
                    .tuples::<(_, _)>()
                    .next()
                    .expect("should be here")
            })
            .map(|(row_val, groups_val)| (parse_row(row_val), parse_groups(groups_val)))
            .map(|(row, damage_groups)| Row { row, damage_groups })
            .map(|row| row.calculate_arrangements())
            .sum(),
        5,
    )
}

fn parse_row(row: &str) -> Vec<Spring> {
    row.chars()
        .map(|char| match char {
            '.' => Spring::Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!("no clue"),
        })
        .collect()
}

fn parse_groups(groups: &str) -> Vec<usize> {
    groups
        .split(',')
        .map(|val| val.parse::<usize>().expect("not a number"))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day12::solve;

    #[test]
    fn test_0() {
        let test = solve("?###? 3".lines()).0;
        assert_eq!(1, test);
    }

    #[test]
    fn test_1() {
        let test = solve("???.### 1,1,3".lines()).0;
        assert_eq!(1, test);
    }

    #[test]
    fn test_2() {
        let test = solve(".??..??...?##. 1,1,3".lines()).0;
        assert_eq!(4, test);
    }

    #[test]
    fn test_3() {
        let test = solve("?#?#?#?#?#?#?#? 1,3,1,6".lines()).0;
        assert_eq!(1, test);
    }

    #[test]
    fn test_4() {
        let test = solve("????.#...#... 4,1,1".lines()).0;
        assert_eq!(1, test);
    }

    #[test]
    fn test_5() {
        let test = solve("????.######..#####. 1,6,5".lines()).0;
        assert_eq!(4, test);
    }

    #[test]
    fn test_6() {
        let test = solve("?###???????? 3,2,1".lines()).0;
        assert_eq!(10, test);
    }
}
