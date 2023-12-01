use std::fs;

pub fn run_p1() {
    let res: u32 = fs::read_to_string("input/day01")
        .expect("Could not open file.")
        .lines()
        .map(|line| line_to_number(line))
        .sum();

    println!("Day 01 Solution Part 1: {}", res);
}

pub fn run_p2() {
    let res: u32 = fs::read_to_string("input/day01")
        .expect("Could not open file.")
        .lines()
        .map(|line| line_to_number_p2(line))
        .sum();

    println!("Day 01 Solution Part 2: {}", res);
}

fn line_to_number(line: &str) -> u32 {
    let mut numbers = line
        .char_indices()
        .filter(|(_idx, val)| *val >= '0' && *val <= '9');
    let first = numbers
        .nth(0)
        .expect(&format!("No first number in line {}", line))
        .1 as u32
        - '0' as u32;
    let last = numbers.nth_back(0).map(|(_, val)| val as u32 - '0' as u32);

    if let Some(last_val) = last {
        first * 10 + last_val
    } else {
        first * 11
    }
}

fn line_to_number_p2(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    let mut remaining = line;
    while remaining.len() > 0 {
        let next_char = remaining.chars().nth(0).expect("no more line");

        if next_char >= '0' && next_char <= '9' {
            if first == None {
                first = Some(next_char as u32 - '0' as u32);
            }
            last = Some(next_char as u32 - '0' as u32);
        } else {
            let mut check = |str_val: &str, int_val: u32| {
                if remaining.starts_with(str_val) {
                    if first == None {
                        first = Some(int_val);
                    }
                    last = Some(int_val);
                    true
                } else {
                    false
                }
            };

            let _ = check("one", 1)
                || check("two", 2)
                || check("three", 3)
                || check("four", 4)
                || check("five", 5)
                || check("six", 6)
                || check("seven", 7)
                || check("eight", 8)
                || check("nine", 9);
        }
        remaining = &remaining[1..];
    }

    first.expect(&format!("No first number in line {}", line)) * 10
        + last.expect(&format!("No second number in line {}", line))
}

#[cfg(test)]
mod day01_test {
    use crate::day01;

    #[test]
    pub fn test_p1() {
        assert_eq!(day01::line_to_number(&"1abc2"), 12);
        assert_eq!(day01::line_to_number(&"pqr3stu8vwx"), 38);
        assert_eq!(day01::line_to_number(&"a1b2c3d4e5f"), 15);
    }

    #[test]
    pub fn test_p2() {
        assert_eq!(day01::line_to_number_p2(&"two1nine"), 29);
        assert_eq!(day01::line_to_number_p2(&"eightwothree"), 83);
    }
}
