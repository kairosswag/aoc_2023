use parse_display::{Display, FromStr};
use std::fs;
use std::time::Instant;

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{from}~{to}")]
struct Brick {
    from: Vector,
    to: Vector,
}

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{x},{y},{z}")]
struct Vector {
    x: usize,
    y: usize,
    z: usize,
}

pub fn run() {
    let file = fs::read_to_string("input/day22").expect("Could not open file.");
    let bricks: Vec<Brick> = file
        .lines()
        .map(|str| str.parse::<Brick>().expect("could not parse line"))
        .collect();
    let now = Instant::now();
    let (res_1, res_2) = (5, 5);
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 22 Solution Part 1: {}", res_1);
    println!("Day 22 Solution Part 2: {}", res_2);
}

fn solve(bricks: &[Brick]) -> usize {
    todo!()
}
