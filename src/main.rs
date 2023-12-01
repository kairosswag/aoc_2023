mod day01;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day01::run_p1();
    day01::run_p2();
    println!("Took {} µs", now.elapsed().as_micros());
}