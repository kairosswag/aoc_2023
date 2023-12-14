use std::fs;
use std::str::Lines;
use std::time::Instant;

use itertools::Itertools;

use crate::day12::Spring::{Damaged, Unknown};

pub fn run() {
    let file = fs::read_to_string("input/day12").expect("Could not open file.");
    let now = Instant::now();
    let (res_1, res_2) = solve(file.lines());
    println!("Solutions took {} µs", now.elapsed().as_micros());
    assert_eq!(7361, res_1);
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

struct Group {
    elements: Vec<Spring>,
    pure: bool,
}

impl Row {
    fn calculate_arrangements(&self) -> usize {
        Self::sub_calculate(&self.row, &self.damage_groups)
    }

    fn calculate_arrangements_expanded(&self) -> usize {
        let mut expanded_row = self.row.clone();
        expanded_row.push(Unknown);
        let mut expanded_row = expanded_row.repeat(5);
        let _ = expanded_row.pop();

        println!("Starting expanded");
        Self::sub_calculate(&expanded_row, &self.damage_groups.repeat(5))
    }

    fn sub_calculate(rest_row: &[Spring], rest_groups: &[usize]) -> usize {
        if rest_groups.len() == 0 {
            if rest_row.iter().any(|filter| *filter == Damaged) {
                return 0;
            }
            return 1;
        } else if rest_row.len() < rest_groups[0] {
            return 0;
        }
        if Self::count_def_blocks(rest_row) > rest_groups.len() {
            return 0;
        }

        let curr_group = rest_groups[0];
        let mut total = 0;
        'outer: for i in 0..=rest_row.len() - curr_group {
            let all_following_may_be_damaged = rest_row[i..i + curr_group]
                .iter()
                .all(|val| *val == Damaged || *val == Unknown);
            let res = match (all_following_may_be_damaged, rest_row.get(i + curr_group)) {
                (true, Some(Unknown)) | (true, Some(Spring::Operational)) => {
                    Self::sub_calculate(&rest_row[i + curr_group + 1..], &rest_groups[1..])
                }
                (true, None) => Self::sub_calculate(&rest_row[i + curr_group..], &rest_groups[1..]),
                (false, _) | (true, Some(Damaged)) => 0,
            };
            if res != 0 {
                total += res;
            }
            if rest_row[i] == Damaged {
                break 'outer;
            }
        }
        total
    }

    fn count_def_blocks(springs: &[Spring]) -> usize {
        let mut count = 0;
        let mut in_block = false;
        for spring in springs {
            match spring {
                Spring::Operational => in_block = false,
                Spring::Damaged => {
                    if in_block == false {
                        count += 1;
                    }
                    in_block = true
                }
                Spring::Unknown => (),
            }
        }
        count
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
    lines
        .map(|line| {
            line.split_whitespace()
                .tuples::<(_, _)>()
                .next()
                .expect("should be here")
        })
        .map(|(row_val, groups_val)| (parse_row(row_val), parse_groups(groups_val)))
        .map(|(row, damage_groups)| Row { row, damage_groups })
        .map(|row| {
            println!("Starting row {:?}", row);
            (
                row.calculate_arrangements(),
                row.calculate_arrangements_expanded(),
            )
        })
        .reduce(|(p1, p2), (q1, q2)| (p1 + q1, p2 + q2))
        .expect("should give result")
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
        let test = solve("?###? 3".lines());
        assert_eq!((1, 1), test);
    }

    #[test]
    fn test_1() {
        let test = solve("???.### 1,1,3".lines());
        assert_eq!((1, 1), test);
    }

    #[test]
    fn test_2() {
        let test = solve(".??..??...?##. 1,1,3".lines());
        assert_eq!((4, 16384), test);
    }

    #[test]
    fn test_3() {
        let test = solve("?#?#?#?#?#?#?#? 1,3,1,6".lines());
        assert_eq!((1, 1), test);
    }

    #[test]
    fn test_4() {
        let test = solve("????.#...#... 4,1,1".lines());
        assert_eq!((1, 16), test);
    }

    #[test]
    fn test_5() {
        let test = solve("????.######..#####. 1,6,5".lines());
        assert_eq!((4, 2500), test);
    }

    #[test]
    fn test_6() {
        let test = solve("?###???????? 3,2,1".lines());
        assert_eq!((10, 506250), test);
    }
}
