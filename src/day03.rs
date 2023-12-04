use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::str::Lines;

pub fn run() {
    let res = solve(fs::read_to_string("input/day03")
        .expect("Could not open file.").lines());
    println!("Day 03 Solution Part 1: {}", res.0);
    println!("Day 03 Solution Part 2: {}", res.1);
}

fn solve(lines: Lines<'_>) -> (u32, u32) {
    let mut special_chars = HashMap::new();
    let mut numbers = HashMap::new();
    for (row, line) in zip(0.., lines) {
        let mut number = Vec::new();
        let mut first_found = None;
        for (col, char) in line.char_indices() {
            match char {
                '.' => {
                    complete_number(&number, row, first_found, &mut numbers);
                    number = Vec::new();
                    first_found = None;
                }
                num @ '0'..='9' => {
                    // special_chars.insert((row as i32, col as i32), num);
                    number.push(num);
                    if first_found == None {
                        first_found = Some(i32::try_from(col).expect("col too large?"));
                    }
                }
                special => {
                    complete_number(&number, row, first_found, &mut numbers);
                    number = Vec::new();
                    first_found = None;
                    special_chars.insert((row as i32, col as i32), special);
                }
            }
        }
        complete_number(&number, row, first_found, &mut numbers);
    }

    let mut res_p1 = 0;
    let mut gear_candidates = HashMap::new();
    for ((row, col), number) in numbers {
        if let Some((special, row, col)) = scan_neighborhood_hit(row, col, number.len() as i32, &special_chars) {
            // println!("found completed number {} @ ({},{}) matching {}", number, row, col, _special);
            let parsed = number.parse::<u32>().expect("could not parse number");

            res_p1 += parsed;

            if special == '*' {
                let entry = gear_candidates.entry((row, col)).or_insert(Vec::new());
                entry.push(parsed);
            }
        }
    }
    let res_p2 = gear_candidates.values().filter(|val| val.len() == 2).map(|val| val[0] * val[1]).sum();
    (res_p1, res_p2)
}

fn scan_neighborhood_hit(row: i32, col: i32, len: i32, special_chars: &HashMap<(i32, i32), char>) -> Option<(char, i32, i32)> {
    // scan top
    if row > 0 {
        for idx in col - 1..=col + len {
            if let Some(value) = special_chars.get(&(row - 1, idx)) {
                return Some((*value, row - 1, idx));
            }
        }
    }
    // scan left/right
    if let Some(value) = special_chars.get(&(row, col - 1)) {
        return Some((*value, row, col - 1));
    }
    if let Some(value) = special_chars.get(&(row, col + len)) {
        return Some((*value, row, col + len));
    }
    // scan bottom
    for idx in col - 1..=col + len {
        if let Some(value) = special_chars.get(&(row + 1, idx)) {
            return Some((*value, row + 1, idx));
        }
    }
    return None;
}

fn complete_number(collected: &[char], row: i32, column: Option<i32>, numbers: &mut HashMap<(i32, i32), String>) {
    if collected.len() == 0 {
        return;
    }
    let column = column.expect("Got characters but no start??");
    let number = collected.iter().collect::<String>().parse::<u32>().expect("Could not parse");
    // println!("found completed number {} @ ({},{})", number, row, column);

    numbers.insert((row, column), collected.iter().collect());
}

#[cfg(test)]
mod test {
    use crate::day03;

    #[test]
    pub fn test_01() {
        let test_input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let res = day03::solve(test_input.lines());
        assert_eq!(res.0, 4361);
        assert_eq!(res.1, 467835);
    }
}