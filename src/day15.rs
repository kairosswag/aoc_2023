use std::fs;
use std::time::Instant;

pub fn run() {
    let file = fs::read_to_string("input/day15").expect("Could not open file.");
    let now = Instant::now();
    let (res_1, res_2) = solve(file.lines().next().unwrap());
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 15 Solution Part 1: {}", res_1);
    println!("Day 15 Solution Part 2: {}", res_2);
}

fn solve(input: &str) -> (usize, usize) {
    let res_1 = input.split(',').map(hash_val).sum();

    let mut lens_boxes = vec![Vec::new(); 256];
    for (ops, bucket) in input.split(',').map(determine_operation) {
        use Operation::*;
        match ops {
            Remove(label) => remove_label(&label, &mut lens_boxes[bucket]),
            Set(label, lens_value) => set(&label, lens_value, &mut lens_boxes[bucket]),
        }
    }

    let mut total = 0;
    for box_no in 0..lens_boxes.len() {
        let lenses = &lens_boxes[box_no];
        for (pos, lens_val) in lenses.iter().map(|(_label, lens_val)| lens_val).enumerate() {
            let focus_power = (box_no + 1) * (pos + 1) * lens_val;
            total += focus_power;
        }
    }

    (res_1, total)
}

fn set(label: &str, lens_val: usize, list: &mut Vec<(String, usize)>) {
    if let Some(idx) = list
        .iter()
        .enumerate()
        .find(|val| label.eq(&val.1 .0))
        .map(|(val, _label)| val)
    {
        list[idx].1 = lens_val;
    } else {
        list.push((String::from(label), lens_val))
    }
}

fn remove_label(label: &str, list: &mut Vec<(String, usize)>) {
    if let Some(idx) = list
        .iter()
        .enumerate()
        .find(|val| label.eq(&val.1 .0))
        .map(|(val, _label)| val)
    {
        list.remove(idx);
    }
}

enum Operation {
    Remove(String),
    Set(String, usize),
}

fn determine_operation(val: &str) -> (Operation, usize) {
    let splits: Vec<_> = val.trim().split(&['=', '-'][..]).collect();
    if splits.len() == 1 || splits[1].is_empty() {
        let label = splits[0];
        let bucket = hash_val(label);
        (Operation::Remove(String::from(label)), bucket)
    } else if splits.len() == 2 {
        let label = splits[0];
        let lens_val = splits[1]
            .parse::<usize>()
            .expect("Could not parse lens val");
        let bucket = hash_val(label);
        (Operation::Set(String::from(label), lens_val), bucket)
    } else {
        println!("val: {val}");
        unreachable!("uh?");
    }
}

fn hash_val(val: &str) -> usize {
    let mut curr_val = 0;
    for char in val.chars() {
        curr_val += char as usize;
        curr_val *= 17;
        curr_val %= 256;
    }

    curr_val
}
