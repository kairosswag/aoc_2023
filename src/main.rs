#![feature(slice_group_by)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day11::run();
    println!("Took {} µs", now.elapsed().as_micros());
}
