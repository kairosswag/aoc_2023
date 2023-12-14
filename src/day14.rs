use crate::day14::Rock::{Cube, Round};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Rock {
    Round,
    Cube,
    None,
}

#[derive(PartialEq)]
struct MirrorMess {
    width: usize,
    height: usize,
    grid: HashMap<(usize, usize), Rock>,
}

pub fn run() {
    let file = fs::read_to_string("input/day14").expect("Could not open file.");
    let now = Instant::now();
    let (res_1, res_2) = solve(file.lines());
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 14 Solution Part 1: {}", res_1);
    println!("Day 14 Solution Part 2: {}", res_2);
}

fn solve(lines: Lines) -> (usize, usize) {
    let mirror_mess = parse_grid(lines);

    let tilted = tilt_north(mirror_mess);

    (calculate_costs(&tilted), 5)
}

fn print_board(mirror_mess: &MirrorMess) {
    for x_val in 0..mirror_mess.width {
        for y_val in 0..mirror_mess.height {
            match mirror_mess.grid.get(&(x_val, y_val)) {
                Some(Cube) => print!("#"),
                Some(Round) => print!("O"),
                None => print!("."),
                _ => unreachable!("none should not be inside"),
            }
        }
        println!()
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

fn calculate_costs(mirror_mess: &MirrorMess) -> usize {
    mirror_mess
        .grid
        .iter()
        .filter_map(|(coord, rock)| (*rock == Round).then(|| coord))
        .map(|(x_val, _y_val)| mirror_mess.height - *x_val)
        .sum()
}

fn tilt_north(mut mirror_mess: MirrorMess) -> MirrorMess {
    let mut columns = vec![Vec::new(); mirror_mess.width];
    for entry in &mirror_mess.grid {
        columns[entry.0 .1].push(entry);
    }

    let mut tilted_grid = HashMap::new();
    for mut col in columns {
        col.sort_by_key(|((x_val, _y_val), _rock)| x_val);

        let mut curr_idx = 0;
        for ((x_pos, y_pos), rock) in col {
            use Rock::*;
            match rock {
                Cube => {
                    tilted_grid.insert((*x_pos, *y_pos), Cube);
                    curr_idx = x_pos + 1;
                }
                Round => {
                    tilted_grid.insert((curr_idx, *y_pos), Round);
                    curr_idx += 1;
                }
                None => (),
            }
        }
    }

    MirrorMess {
        width: mirror_mess.width,
        height: mirror_mess.height,
        grid: tilted_grid,
    }
}

fn parse_grid(lines: Lines) -> MirrorMess {
    use Rock::*;
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (x_pos, line) in lines.enumerate() {
        width = line.len();
        height += 1;
        for (y_pos, char) in line.char_indices() {
            match char {
                'O' => {
                    grid.insert((x_pos, y_pos), Round);
                }
                '#' => {
                    grid.insert((x_pos, y_pos), Cube);
                }
                _ => (),
            }
        }
    }
    MirrorMess {
        width,
        height,
        grid,
    }
}

#[test]
fn test() {
    let test_input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    let test_res = solve(test_input.lines());
    assert_eq!((136, 5), test_res);
}
