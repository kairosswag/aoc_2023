use crate::day10::Direction::{East, South, West};
use crate::day10::Pipe::{
    Ground, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start, Vertical,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::Lines;

pub fn run() {
    let file = fs::read_to_string("input/day10").expect("Could not open file.");
    let map = parse_map(file.lines());
    let res_p1 = solve(&map);
    println!("Day 10 Solution Part 1: {}", res_p1);
    println!("Day 10 Solution Part 2: {}", 5);
}

fn parse_map(lines: Lines) -> HashMap<(i32, i32), Pipe> {
    let mut pipe_map = HashMap::new();
    for (y_val, line) in lines.enumerate() {
        for (x_val, char) in line.trim().char_indices() {
            pipe_map.insert((x_val as i32, y_val as i32), Pipe::from_char(&char));
        }
    }

    pipe_map
}

fn solve(pipe_map: &HashMap<(i32, i32), Pipe>) -> usize {
    let (start_pos, _) = pipe_map
        .iter()
        .filter(|(_, pipe)| **pipe == Start)
        .next()
        .expect("Could not find start");

    let starting_direction = determine_start_directions(start_pos, &pipe_map).0;

    let mut curr_pos = *start_pos;
    let mut curr_dir = starting_direction;
    let mut path = Vec::new();

    loop {
        let (next_pos, next_direction) = perform_step(&curr_pos, &curr_dir, pipe_map);
        if next_pos == *start_pos {
            break;
        }

        path.push(next_pos);
        curr_pos = next_pos;
        curr_dir = next_direction;
    }

    (path.len() + 1) / 2
}

fn perform_step(
    curr_pos: &(i32, i32),
    direction: &Direction,
    pipe_map: &HashMap<(i32, i32), Pipe>,
) -> ((i32, i32), Direction) {
    let next_pos = direction.move_dir(curr_pos);
    let next_pipe = pipe_map.get(&next_pos).expect("cannot move there");
    let next_dir = next_pipe
        .change_dir_safe(direction)
        .expect("Cannot enter that pipe from that side");

    (next_pos, next_dir)
}

fn determine_start_directions(
    start_pos: &(i32, i32),
    pipe_map: &&HashMap<(i32, i32), Pipe>,
) -> (Direction, Direction) {
    use Direction::*;
    [North, East, South, West]
        .iter()
        .map(|direction| (direction, direction.move_dir(start_pos)))
        .map(|(dir, n_pos)| (dir, pipe_map.get(&n_pos).expect("should be within")))
        .filter_map(|(dir, pipe)| pipe.change_dir_safe(dir))
        .tuples()
        .next()
        .expect("could not find start dir")
}

#[derive(PartialEq)]
enum Pipe {
    Horizontal,
    Vertical,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_dir(&self, (start_x, start_y): &(i32, i32)) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (*start_x, start_y - 1),
            East => (start_x + 1, *start_y),
            South => (*start_x, start_y + 1),
            West => (start_x - 1, *start_y),
        }
    }
}

impl Pipe {
    fn from_char(character: &char) -> Self {
        match character {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthToEast,
            'J' => NorthToWest,
            '7' => SouthToWest,
            'F' => SouthToEast,
            '.' => Ground,
            'S' => Start,
            val => {
                println!("val: {}", val);
                unreachable!("eh?")
            }
        }
    }

    fn change_dir_safe(&self, dir: &Direction) -> Option<Direction> {
        use Direction::*;
        use Pipe::*;
        match (self, dir) {
            (Vertical, North) | (NorthToEast, West) | (NorthToWest, East) => Some(North),
            (Vertical, South) | (SouthToWest, East) | (SouthToEast, West) => Some(South),
            (Horizontal, East) | (NorthToEast, South) | (SouthToEast, North) => Some(East),
            (Horizontal, West) | (NorthToWest, South) | (SouthToWest, North) => Some(West),
            (Start, _) => Some(North), // eh..
            _ => None,
        }
    }
}
