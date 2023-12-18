use crate::day17::Direction::Left;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Read;
use std::ops::{Range, RangeInclusive};
use std::time::Instant;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Face {
    Horizontal,
    Vertical,
}

impl Face {
    fn other(&self) -> Self {
        use Face::*;
        match self {
            Horizontal => Vertical,
            Vertical => Horizontal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    cost: usize,
    target: usize,
    facing: Face,
}

#[derive(Debug)]
struct InputMap<'a> {
    values: &'a [u8],
    width: usize,
    height: usize,
}

impl Path {
    fn new(cost: usize, target: usize, facing: Face) -> Self {
        Path {
            cost,
            target,
            facing,
        }
    }
}

pub fn run() {
    let mut input = Vec::new();
    let mut file = File::open("input/day17").expect("Could not open file.");
    file.read_to_end(&mut input).expect("could not read");
    let now = Instant::now();
    let (res_1, res_2) = solve(&input);
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 17 Solution Part 1: {}", res_1);
    println!("Day 17 Solution Part 2: {}", res_2);
}

fn solve(input: &[u8]) -> (usize, usize) {
    let p1 = solve_p1(input, 1, 3);
    let p2 = solve_p1(input, 4, 10);
    (p1, p2)
}

fn solve_p1(input: &[u8], range_start: usize, range_end_incl: usize) -> usize {
    let mut map = initialize_input_map(input);
    let mut paths_available = BinaryHeap::new();
    for path in next_targets(0, Face::Horizontal, 0, range_start, range_end_incl, &map) {
        paths_available.push(Reverse(path));
    }
    for path in next_targets(0, Face::Vertical, 0, range_start, range_end_incl, &map) {
        paths_available.push(Reverse(path));
    }

    let mut curr_found = HashMap::new();

    while let Some(Reverse(path)) = paths_available.pop() {
        if let Some(_) = curr_found.get(&(path.target, path.facing)) {
            continue;
        } else {
            if path.target == (map.width * map.height) - 2 {
                return path.cost;
            }
            curr_found.insert((path.target, path.facing), path.cost);
            let next_targets = next_targets(
                path.target,
                path.facing,
                path.cost,
                range_start,
                range_end_incl,
                &map,
            );
            for next in next_targets {
                if !curr_found.contains_key(&(next.target, next.facing)) {
                    paths_available.push(Reverse(next));
                }
            }
        }
    }

    panic!("no result found;")
}

fn next_targets(
    curr_pos: usize,
    facing: Face,
    cost: usize,
    range_start: usize,
    range_end_incl: usize,
    map: &InputMap,
) -> Vec<Path> {
    use Direction::*;
    let directions = match facing {
        Face::Vertical => [Up, Down],
        Face::Horizontal => [Left, Right],
    };

    let mut next_target = Vec::new();
    for direction in directions {
        let mut path_cost = cost;
        let mut mv_pos = curr_pos;
        for idx in 1..=range_end_incl {
            if let Some(pos) = step(mv_pos, &direction, map.width, map.height) {
                mv_pos = pos;
                path_cost += (map.values[mv_pos] - '0' as u8) as usize;
                if idx >= range_start {
                    let new_path = Path::new(path_cost, mv_pos, facing.other());
                    next_target.push(new_path);
                }
            }
        }
    }

    next_target
}

struct Dimensions {
    width: usize,
    height: usize,
}
fn initialize_input_map(input: &[u8]) -> InputMap {
    let width = input
        .iter()
        .enumerate()
        .find_or_first(|(_, val)| **val as char == '\n');

    let width = width
        .expect("Could not determine width: no linebreak found")
        .0
        + 1;

    let height = (input.len() + 1) / width;

    InputMap {
        values: input,
        width,
        height,
    }
}

fn step(position: usize, direction: &Direction, width: usize, height: usize) -> Option<usize> {
    use Direction::*;
    match direction {
        Up => {
            if position < width {
                None
            } else {
                Some(position - width)
            }
        }
        Down => {
            let next_pos = position + width;
            if next_pos >= width * height {
                None
            } else {
                Some(next_pos)
            }
        }
        Left => {
            if position % width == 0 {
                None
            } else {
                Some(position - 1)
            }
        }
        Right => {
            if position % width == width - 2 {
                None
            } else if position % width == width - 1 {
                unreachable!("this is not allowed, this is position contains a backslash");
            } else {
                Some(position + 1)
            }
        }
    }
}

#[test]
fn test() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    let res = solve(input.as_bytes());
    assert_eq!((102, 5), res);
}

#[test]
fn test_step_function() {
    use Direction::*;
    // assert_eq!(None, step(0, &Left, 5, 5));
    // assert_eq!(None, step(4, &Right, 5, 5));
    // assert_eq!(Some(4), step(3, &Right, 5, 1));
    assert_eq!(None, step(26, &Right, 14, 15));
    assert_eq!(Some(180), step(179, &Right, 14, 15));
    assert_eq!(Some(180), step(166, &Down, 14, 15));
}
