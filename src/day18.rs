use ascii::AsAsciiStr;
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{direction} {value} ({color_code}")]
struct DigInstruction {
    direction: Direction,
    value: usize,
    color_code: LongInstruction,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{}")]
enum Direction {
    #[display("R")]
    Right,
    #[display("L")]
    Left,
    #[display("U")]
    Up,
    #[display("D")]
    Down,
}

#[derive(PartialEq, Debug, Display)]
#[display("{direction}{value}")]
struct LongInstruction {
    direction: Direction,
    value: usize,
}

impl FromStr for LongInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = usize::from_str_radix(&input[1..6], 16).expect(&format!(
            "Could not parse number {:?}",
            input[1..6].as_ascii_str()
        ));
        let direction = match input.chars().nth(6).expect("meh") {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!("apparently did not parse direction"),
        };
        Ok(LongInstruction { direction, value })
    }
}

pub fn run() {
    let file = fs::read_to_string("input/day18").expect("Could not open file.");
    let dig_instructions: Vec<DigInstruction> = file
        .lines()
        .map(|str| str.parse::<DigInstruction>().expect("could not parse line"))
        .collect();
    let now = Instant::now();
    let (res_1, res_2) = solve(&dig_instructions);
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 18 Solution Part 1: {}", res_1);
    println!("Day 18 Solution Part 2: {}", res_2);
}

fn solve(instructions: &[DigInstruction]) -> (usize, usize) {
    let mut dug = HashMap::new();
    let mut curr_pos = (0, 0);
    let mut minmax = (0, 0, 0, 0);
    for instr in instructions {
        for _ in 0..instr.value {
            match instr.direction {
                Direction::Up => curr_pos = (curr_pos.0 - 1, curr_pos.1),
                Direction::Down => curr_pos = (curr_pos.0 + 1, curr_pos.1),
                Direction::Right => curr_pos = (curr_pos.0, curr_pos.1 + 1),
                Direction::Left => curr_pos = (curr_pos.0, curr_pos.1 - 1),
            }
            minmax = (
                minmax.0.min(curr_pos.0),
                minmax.1.min(curr_pos.1),
                minmax.2.max(curr_pos.0),
                minmax.3.max(curr_pos.1),
            );
            dug.insert(curr_pos, 0);
        }
    }

    let mut borders = HashSet::new();
    let mut color = 0;
    for x_val in minmax.0..minmax.2 {
        for y_val in minmax.1..minmax.3 {
            color += 1;
            if !dug.contains_key(&(x_val, y_val)) {
                color_map(&mut dug, (x_val, y_val), color, &mut borders, minmax);
            }
        }
    }

    println!("max_color {}", color);
    println!("minmax dbg {:?}", minmax);
    println!(
        "minmax val {}",
        (minmax.2 - minmax.0) * (minmax.3 - minmax.1)
    );
    let p_1 = dug
        .values()
        .filter(|value| !borders.contains(value))
        .count();

    (p_1, 5)
}

fn color_map(
    dug: &mut HashMap<(i32, i32), usize>,
    pos: (i32, i32),
    color: usize,
    borders: &mut HashSet<usize>,
    minmax: (i32, i32, i32, i32),
) {
    let mut fill_map = Vec::new();
    fill_map.push(pos);
    while let Some((x_val, y_val)) = fill_map.pop() {
        dug.insert((x_val, y_val), color);
        let neighbors = [
            (x_val + 1, y_val),
            (x_val - 1, y_val),
            (x_val, y_val + 1),
            (x_val, y_val - 1),
        ];
        for neighbor in neighbors {
            if neighbor.0 < minmax.0
                || neighbor.1 < minmax.1
                || neighbor.0 > minmax.2
                || neighbor.1 > minmax.3
            {
                borders.insert(color);
            } else if dug.contains_key(&neighbor) {
                continue;
            } else {
                fill_map.push(neighbor);
            }
        }
    }
}
