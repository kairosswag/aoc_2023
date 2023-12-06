mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day06::run();
    println!("Took {} µs", now.elapsed().as_micros());
}
