use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::time::Instant;

use itertools::Itertools;

use crate::day14::Rock::{Cube, Round};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Rock {
    Round,
    Cube,
}

#[derive(PartialEq, Clone)]
struct MirrorMess {
    width: usize,
    height: usize,
    grid: HashMap<(usize, usize), Rock>,
}

#[derive(PartialOrd, PartialEq, Hash)]
struct Comparable(Vec<usize>);

impl MirrorMess {
    fn to_comparable(&self) -> Vec<usize> {
        let mut compare = Vec::with_capacity(3);
        for ((x_val, y_val), rock) in self.grid.iter().sorted() {
            compare.push(*x_val);
            compare.push(*y_val);
            match rock {
                Round => compare.push(1),
                Cube => compare.push(2),
            }
        }
        compare
    }

    fn tilt_north(&self) -> MirrorMess {
        let mut columns = vec![Vec::new(); self.width];
        for entry in &self.grid {
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
                }
            }
        }

        MirrorMess {
            width: self.width,
            height: self.height,
            grid: tilted_grid,
        }
    }

    fn rotate(&self) -> MirrorMess {
        let mut tilted = HashMap::new();
        for ((x_val, y_val), rock) in &self.grid {
            let x_tilt = *y_val;
            let y_tilt = self.height - x_val - 1;

            tilted.insert((x_tilt, y_tilt), *rock);
        }

        MirrorMess {
            width: self.height,
            height: self.width,
            grid: tilted,
        }
    }
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

    let tilted = mirror_mess.tilt_north();

    let mut found_maps = HashMap::new();
    let mut current_map = mirror_mess;
    let mut curr_cycle: usize = 1;
    let cycle_repeat;
    loop {
        let current_compare = current_map.to_comparable();
        if !found_maps.contains_key(&current_compare) {
            let tilted = current_map.tilt_north();
            let west = tilted.rotate();
            let west_tilted = west.tilt_north();
            let south = west_tilted.rotate();
            let south_tilted = south.tilt_north();
            let east = south_tilted.rotate();
            let east_tilted = east.tilt_north();
            current_map = east_tilted.rotate();
            found_maps.insert(current_compare, (curr_cycle, current_map.clone()));
            curr_cycle += 1;
        } else {
            cycle_repeat = found_maps.get(&current_compare).expect("wait, what?").0;
            break;
        }
    }
    let repeat_frequency = curr_cycle - cycle_repeat;
    let remaining = (1000000000 - cycle_repeat) % repeat_frequency;
    let res_2 = found_maps
        .values()
        .filter_map(|(cycle_no, val)| {
            (*cycle_no == cycle_repeat + remaining).then(|| calculate_costs(val))
        })
        .next()
        .expect("no result :(");

    (calculate_costs(&tilted), res_2)
}

fn calculate_costs(mirror_mess: &MirrorMess) -> usize {
    mirror_mess
        .grid
        .iter()
        .filter_map(|(coord, rock)| (*rock == Round).then(|| coord))
        .map(|(x_val, _y_val)| mirror_mess.height - *x_val)
        .sum()
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
    assert_eq!((136, 64), test_res);
}
