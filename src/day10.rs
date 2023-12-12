use crate::day10::Direction::{East, South, West};
use crate::day10::Pipe::{
    Ground, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start, Vertical,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Lines;

pub fn run() {
    let file = fs::read_to_string("input/day10").expect("Could not open file.");
    let map = parse_map(file.lines());
    let (res_p1, res_p2) = solve(map);
    println!("Day 10 Solution Part 1: {}", res_p1);
    println!("Day 10 Solution Part 2: {}", res_p2);
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

fn solve(mut pipe_map: HashMap<(i32, i32), Pipe>) -> (usize, usize) {
    let (start_pos, _) = pipe_map
        .iter()
        .filter(|(_, pipe)| **pipe == Start)
        .next()
        .expect("Could not find start");

    let starting_directions = determine_start_directions(start_pos, &pipe_map);

    let mut curr_pos = *start_pos;
    let mut curr_dir = starting_directions.0;
    let mut path = Vec::new();

    loop {
        let (next_pos, next_direction) = perform_step(&curr_pos, &curr_dir, &pipe_map);
        if next_pos == *start_pos {
            break;
        }

        path.push(next_pos);
        curr_pos = next_pos;
        curr_dir = next_direction;
    }

    let res_1 = (path.len() + 1) / 2;

    path.push(*start_pos);
    pipe_map.insert(
        *start_pos,
        Pipe::determine_start_pipe(&starting_directions.0, &starting_directions.1),
    );
    let (min_x, min_y, max_x, max_y) = path.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(min_x, min_y, max_x, max_y), (path_x, path_y)| {
            (
                min_x.min(*path_x),
                min_y.min(*path_y),
                max_x.max(*path_x),
                max_y.max(*path_y),
            )
        },
    );

    let path_lookup: HashSet<&(i32, i32)> = path.iter().collect();

    let candidates: Vec<&(i32, i32)> = pipe_map
        .iter()
        .filter_map(|(val, _)| (path_lookup.get(val) == None).then(|| val))
        .filter(|(elem_x, elem_y)| {
            *elem_x >= min_x && *elem_y >= min_y && *elem_x <= max_x && *elem_y <= max_y
        })
        .collect();

    let mut res_2 = 0;
    for (can_x, can_y) in candidates {
        let left = path
            .iter()
            .filter(|(path_x, path_y)| path_x < can_x && path_y == can_y)
            .sorted_by(|first, second| second.0.cmp(&first.0))
            .collect::<Vec<&(i32, i32)>>();

        if determine_inside(&left, &pipe_map) {
            res_2 += 1;
        }
    }

    (res_1, res_2)
}

fn determine_inside(
    left_nodes_sorted: &[&(i32, i32)],
    pipe_map: &HashMap<(i32, i32), Pipe>,
) -> bool {
    let mut inside = false;
    let mut last_found = None;

    for node in left_nodes_sorted {
        let pipe = pipe_map.get(node).expect("wat");
        use Pipe::*;
        match (pipe, &last_found) {
            (Vertical, None) => {
                inside = !inside;
            }
            (NorthToWest, None) => {
                last_found = Some(NorthToWest);
            }
            (SouthToWest, None) => {
                last_found = Some(SouthToWest);
            }
            (SouthToEast, Some(NorthToWest)) => {
                inside = !inside;
                last_found = None;
            }
            (NorthToEast, Some(SouthToWest)) => {
                inside = !inside;
                last_found = None;
            }
            (SouthToEast, Some(SouthToWest)) => {
                last_found = None;
            }
            (NorthToEast, Some(NorthToWest)) => {
                last_found = None;
            }
            (Horizontal, Some(_)) => (),

            (other, val) => {
                println!("{:?} , {:?}", other, val);
                unreachable!("uh?");
            }
        }
    }
    inside
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
    pipe_map: &HashMap<(i32, i32), Pipe>,
) -> (Direction, Direction) {
    use Direction::*;
    [North, East, South, West]
        .iter()
        .map(|direction| (direction, direction.move_dir(start_pos)))
        .filter_map(|(dir, n_pos)| pipe_map.get(&n_pos).map(|val| (dir, val)))
        .filter_map(|(dir, pipe)| pipe.change_dir_safe(dir).map(|_| *dir))
        .tuples()
        .next()
        .expect("could not find start dir")
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug, Copy, Clone)]
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

    fn determine_start_pipe(a: &Direction, b: &Direction) -> Pipe {
        use Direction::*;
        match (a, b) {
            (North, South) | (South, North) => Vertical,
            (West, South) | (South, West) => SouthToWest,
            (East, South) | (South, East) => SouthToWest,
            _ => {
                unimplemented!("not used for my or test input yet, todo if i wanna try more inputs")
            }
        }
    }
}

#[cfg(test)]
mod test_p10 {
    use crate::day10::{parse_map, solve};

    #[test]
    fn test_1() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        let map = parse_map(input.lines());
        let (_, res_p2) = solve(map);
        assert_eq!(4, res_p2);
    }

    #[test]
    fn test_3() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        let map = parse_map(input.lines());
        let (_, res_p2) = solve(map);
        assert_eq!(10, res_p2);
    }
}
