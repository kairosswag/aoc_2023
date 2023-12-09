use std::fs;

pub fn run() {
    let file = fs::read_to_string("input/day09").expect("Could not open file.");
    let mut lines = file.lines();
    let res_1: i32 = lines.map(|line| calc_next_val(line)).sum();
    let res = (res_1, 5);
    println!("Day 09 Solution Part 1: {}", res.0);
    println!("Day 09 Solution Part 2: {}", res.1);
}

fn calc_next_val(line: &str) -> i32 {
    let values: Vec<i32> = line
        .split_whitespace()
        .map(|val| val.parse::<i32>().expect("could not parse"))
        .collect();

    let res_derived = rec_derive_return_last(&values);
    values.last().expect("meh...") + res_derived
}

fn rec_derive_return_last(values: &[i32]) -> i32 {
    let derived: Vec<i32> = values
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    if derived.iter().filter(|val| **val != 0).count() == 0 {
        0
    } else {
        let res_derived = rec_derive_return_last(&derived);
        derived.last().expect("eh...") + res_derived
    }
}
