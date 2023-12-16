#![feature(slice_group_by)]

use std::env;
use std::time::Instant;

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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let args: Vec<String> = env::args().collect();
    let now = Instant::now();
    let to_match = if args.len() > 1 {
        args[1].as_str()
    } else {
        "today"
    };
    match to_match {
        "day01" => day01::run(),
        "day02" => day02::run(),
        "day03" => day03::run(),
        "day04" => day04::run(),
        "day05" => day05::run(),
        "day06" => day06::run(),
        "day07" => day07::run(),
        "day08" => day08::run(),
        "day09" => day09::run(),
        "day10" => day10::run(),
        "day11" => day11::run(),
        "day12" => day12::run(),
        "day13" => day13::run(),
        "day14" => day14::run(),
        "day15" => day15::run(),
        "day16" | "today" => day16::run(),
        _ => unreachable!("Someone forgot to add the day."),
    }
    println!("Total took {} µs", now.elapsed().as_micros());
}
