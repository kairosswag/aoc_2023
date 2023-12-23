use std::fs;
use std::time::Instant;

pub fn run() {
    let file = fs::read_to_string("input/day22").expect("Could not open file.");
    let bricks: Vec<crate::day22::Brick> = file
        .lines()
        .map(|str| {
            str.parse::<crate::day22::Brick>()
                .expect("could not parse line")
        })
        .collect();
    let now = Instant::now();
    let (res_1, res_2) = (5, 5);
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 22 Solution Part 1: {}", res_1);
    println!("Day 22 Solution Part 2: {}", res_2);
}

fn solve(bricks: &[crate::day22::Brick]) -> usize {
    todo!()
}
