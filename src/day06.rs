use std::fs;
use std::iter::zip;

pub fn run() {
    let file = fs::read_to_string("input/day06").expect("Could not open file.");
    let mut lines = file.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().unwrap())
        .collect();

    let res_1 = zip(times, distances)
        .map(|(time, distance)| calc_winning_amount(time, distance))
        .fold(1, |a, b| a * b);

    let mut lines_2 = file.lines();
    let times_p2 = lines_2
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|line| line.replace(' ', ""))
        .map(|no_wsp| no_wsp.parse::<u64>().unwrap())
        .next()
        .unwrap();
    let distances_p2 = lines_2
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|line| line.replace(' ', ""))
        .map(|no_wsp| no_wsp.parse::<u64>().unwrap())
        .next()
        .unwrap();

    let res_2 = calc_winning_amount(times_p2, distances_p2);
    let res = (res_1, res_2);
    println!("Day 06 Solution Part 1: {}", res.0);
    println!("Day 06 Solution Part 2: {}", res.1);
}

fn calc_winning_amount(time: u64, record_dist: u64) -> u64 {
    let mut counter = 0;
    for charging in 1..time {
        // exclude first and last on purpose
        if charging as u64 * ((time - charging) as u64) > record_dist as u64 {
            counter += 1;
        }
    }
    counter
}

#[test]
fn test() {
    let test_input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    calc_winning_amount(7, 9);
}
