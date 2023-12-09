use std::fs;

pub fn run() {
    let file = fs::read_to_string("input/day09").expect("Could not open file.");
    let lines = file.lines();
    let (sol_prev, sol_next): (i32, i32) = lines
        .map(|line| calc_prev_next_val(line))
        .reduce(|(a_p, a_n), (b_p, b_n)| (a_p + b_p, a_n + b_n))
        .unwrap();
    println!("Day 09 Solution Part 1: {}", sol_next);
    println!("Day 09 Solution Part 2: {}", sol_prev);
}

fn calc_prev_next_val(line: &str) -> (i32, i32) {
    let values: Vec<i32> = line
        .split_whitespace()
        .map(|val| val.parse::<i32>().expect("could not parse"))
        .collect();

    let (res_prev_derived, res_next_derived) = rec_derive_return_last(&values);
    (
        values.first().expect("nop...") - res_prev_derived,
        values.last().expect("meh...") + res_next_derived,
    )
}

fn rec_derive_return_last(values: &[i32]) -> (i32, i32) {
    let derived: Vec<i32> = values
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    if derived.iter().filter(|val| **val != 0).count() == 0 {
        (0, 0)
    } else {
        let (res_prev_derived, res_next_derived) = rec_derive_return_last(&derived);
        (
            derived.first().expect("nop...") - res_prev_derived,
            derived.last().expect("meh...") + res_next_derived,
        )
    }
}

#[test]
fn test_values() {
    let res = calc_prev_next_val("10 13 16 21 30 45");
    assert_eq!((5, 68), res);
}
