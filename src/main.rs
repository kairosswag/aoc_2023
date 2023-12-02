mod day01;
mod day02;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day02::run();
    println!("Took {} µs", now.elapsed().as_micros());
}
