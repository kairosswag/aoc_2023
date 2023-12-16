use crate::day16::Direction::{Down, Left, Right, Up};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

const MAP_WIDTH: usize = 111; // including newline
const MAP_HEIGHT: usize = 110;
const MIRROR_BS: u8 = '\\' as u8;
const MIRROR_FS: u8 = '/' as u8;
const SPLIT_HOR: u8 = '-' as u8;
const SPLIT_VER: u8 = '|' as u8;
const EMPTY: u8 = '.' as u8;

pub fn run() {
    let mut input = Vec::new();
    let mut file = File::open("input/day16").expect("Could not open file.");
    file.read_to_end(&mut input).expect("could not read");
    let now = Instant::now();
    let (res_1, res_2) = solve(&input);
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 16 Solution Part 1: {}", res_1);
    println!("Day 16 Solution Part 2: {}", res_2);
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(input: &[u8]) -> (usize, usize) {
    let res_1 = start_beam(0, Right, input);
    let res_2 = moar_energy(input);

    (res_1, res_2)
}

fn moar_energy(map: &[u8]) -> usize {
    let last_row = MAP_WIDTH * (MAP_HEIGHT - 1);
    let mut max = 0;
    for idx in 0..(MAP_WIDTH - 1) {
        let res = start_beam(idx, Down, map);
        max = max.max(res);
        let res = start_beam(last_row + idx, Up, map);
        max = max.max(res);
    }
    for idx in 0..MAP_HEIGHT {
        let res = start_beam(idx * MAP_WIDTH, Right, map);
        max = max.max(res);
        let res = start_beam(idx * MAP_WIDTH + (MAP_WIDTH - 2), Left, map);
        max = max.max(res);
    }
    max
}

fn start_beam(position: usize, direction: Direction, map: &[u8]) -> usize {
    let mut energized = HashSet::new();

    run_beam(position, direction, map, &mut energized);
    energized.len()
}

fn run_beam(position: usize, direction: Direction, map: &[u8], energized: &mut HashSet<usize>) {
    let mut position = position;
    let mut direction = direction;
    loop {
        let mut additional_direction = None;
        let next_direction = match map[position] {
            MIRROR_BS => {
                energized.insert(position);
                match direction {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down,
                }
            }
            MIRROR_FS => {
                energized.insert(position);
                match direction {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up,
                }
            }
            SPLIT_HOR => {
                if energized.contains(&position) {
                    return;
                }
                if direction == Up || direction == Down {
                    energized.insert(position);
                    additional_direction = Some(Left);
                    Right
                } else {
                    energized.insert(position);
                    direction
                }
            }
            SPLIT_VER => {
                if energized.contains(&position) {
                    return;
                }
                if direction == Left || direction == Right {
                    energized.insert(position);
                    additional_direction = Some(Up);
                    Down
                } else {
                    energized.insert(position);
                    direction
                }
            }
            EMPTY => {
                energized.insert(position);
                direction
            }

            _ => {
                println!("position {}, char {}", position, map[position]);
                unreachable!()
            }
        };

        if let Some(split_beam) = additional_direction {
            if let Some(next_pos) = step(position, &split_beam) {
                run_beam(next_pos, split_beam, map, energized);
            }
        }

        if let Some(next_pos) = step(position, &next_direction) {
            direction = next_direction;
            position = next_pos;
        } else {
            return;
        }
    }
}

fn step(position: usize, direction: &Direction) -> Option<usize> {
    match direction {
        Up => {
            if position < MAP_WIDTH {
                None
            } else {
                Some(position - MAP_WIDTH)
            }
        }
        Down => {
            let next_pos = position + MAP_WIDTH;
            if next_pos >= MAP_WIDTH * MAP_HEIGHT {
                None
            } else {
                Some(next_pos)
            }
        }
        Left => {
            if position % MAP_WIDTH == 0 {
                None
            } else {
                Some(position - 1)
            }
        }
        Right => {
            if position % MAP_WIDTH == MAP_WIDTH - 2 {
                None
            } else {
                Some(position + 1)
            }
        }
    }
}

#[test]
fn test() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    let res = solve(input.as_bytes());
    assert_eq!((46, 5), res);
}
