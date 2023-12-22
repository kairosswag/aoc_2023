use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use ascii::AsAsciiStr;
use itertools::{chain, Itertools};
use parse_display::{Display, FromStr};

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
    value: i32,
}

impl FromStr for LongInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = i32::from_str_radix(&input[1..6], 16).expect(&format!(
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

#[derive(Copy, Clone, Debug, Hash)]
struct Edge {
    height: i32,
    start_idx: i32,
    end_idx: i32,
}

impl Eq for Edge {}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
            && self.start_idx == other.start_idx
            && self.end_idx == other.end_idx
    }
}

impl PartialOrd<Self> for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        let height_eq = self.height.cmp(&other.height);
        if Ordering::Equal == height_eq {
            let start_idx_eq = self.start_idx.cmp(&other.start_idx);
            if Ordering::Equal == start_idx_eq {
                self.end_idx.cmp(&other.end_idx)
            } else {
                start_idx_eq
            }
        } else {
            height_eq
        }
    }
}

impl Edge {
    fn new_horizontal(a: (i32, i32), b: (i32, i32)) -> Self {
        assert_eq!(a.0, b.0, "wtfx??");
        let height = a.0;
        let start_idx = a.1.min(b.1);
        let end_idx = a.1.max(b.1);
        Edge {
            height,
            start_idx,
            end_idx,
        }
    }

    fn new_vertical(a: (i32, i32), b: (i32, i32)) -> Self {
        assert_eq!(a.1, b.1, "wtfxx??");
        let height = a.1;
        let start_idx = a.0.min(b.0);
        let end_idx = a.0.max(b.0);
        Edge {
            height,
            start_idx,
            end_idx,
        }
    }

    fn cut(&self, new_end: i32) -> Option<Edge> {
        if self.start_idx < new_end {
            Some(Edge {
                height: self.height,
                start_idx: self.start_idx,
                end_idx: new_end,
            })
        } else {
            None
        }
    }
}

pub fn run() {
    let file = fs::read_to_string("input/day18").expect("Could not open file.");
    let dig_instructions: Vec<DigInstruction> = file
        .lines()
        .map(|str| str.parse::<DigInstruction>().expect("could not parse line"))
        .collect();
    let now = Instant::now();
    let (res_1, res_2) = (solve(&dig_instructions), solve_p2(&dig_instructions));
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 18 Solution Part 1: {}", res_1);
    println!("Day 18 Solution Part 2: {}", res_2);
}

fn solve_p2(instructions: &[DigInstruction]) -> usize {
    let mut horizontal_edges: BinaryHeap<Edge> = BinaryHeap::new();
    let mut vertical_edges = HashSet::new();
    let mut curr_pos: (i32, i32) = (0, 0);
    for instr in instructions {
        let LongInstruction { direction, value } = &instr.color_code;
        let next_pos = match direction {
            Direction::Up => (curr_pos.0 - value, curr_pos.1),
            Direction::Down => (curr_pos.0 + value, curr_pos.1),
            Direction::Right => (curr_pos.0, curr_pos.1 + value),
            Direction::Left => (curr_pos.0, curr_pos.1 - value),
        };

        match direction {
            Direction::Up | Direction::Down => {
                vertical_edges.insert(Edge::new_vertical(curr_pos, next_pos));
            }
            _ => horizontal_edges.push(Edge::new_horizontal(curr_pos, next_pos)),
        }
        curr_pos = next_pos
    }

    let mut total = 0;

    while !vertical_edges.is_empty() {
        // print_vert_edges(&vertical_edges);
        // print_hor_edges(&mut horizontal_edges);
        let current = horizontal_edges.pop().expect("i did something wrong");
        let left = find_and_remove(current.height, current.start_idx, &mut vertical_edges);
        let right = find_and_remove(current.height, current.end_idx, &mut vertical_edges);

        let (relevant_hor, found_height) =
            find_next_relevant_horizontal(&left, &right, &mut horizontal_edges);

        // println!("left: {:?}", left);
        // println!("right: {:?}", right);
        // println!("relevant hor: {:?}", relevant_hor);

        let width = (current.end_idx - current.start_idx + 1) as usize;
        let height = (current.height - found_height) as usize;
        let res = width * height;

        let mut relevant_vert_edges = find_vert_edges(&relevant_hor, &vertical_edges);
        relevant_vert_edges
            .iter()
            .for_each(|edge| assert!(vertical_edges.remove(edge), "edge did not exist"));

        if let Some(new_edge) = left.cut(found_height) {
            relevant_vert_edges.push(new_edge);
        }
        if let Some(new_edge) = right.cut(found_height) {
            relevant_vert_edges.push(new_edge);
        }

        let new_horizontals: Vec<Edge> = relevant_vert_edges
            .iter_mut()
            .sorted()
            .tuples()
            .map(|(l_edge, r_edge)| {
                let height = found_height;
                if l_edge.start_idx != found_height {
                    l_edge.end_idx = height;
                }
                if r_edge.start_idx != found_height {
                    r_edge.end_idx = height;
                }
                let start_idx = l_edge.height.min(r_edge.height);
                let end_idx = l_edge.height.max(r_edge.height);

                Edge {
                    height,
                    start_idx,
                    end_idx,
                }
            })
            .collect();

        for edge in relevant_vert_edges {
            vertical_edges.insert(edge);
        }

        let broad_total = relevant_hor.iter().fold(current, |mut acc, b| {
            acc.start_idx = acc.start_idx.min(b.start_idx);
            acc.end_idx = acc.end_idx.max(b.end_idx);
            acc
        });
        let mut broad_total = (broad_total.end_idx - broad_total.start_idx + 1) as usize;

        for edge in new_horizontals {
            broad_total -= (edge.end_idx - edge.start_idx + 1) as usize;
            horizontal_edges.push(edge);
        }
        total += res;
        total += broad_total;
    }
    total
}

fn find_next_relevant_horizontal(
    left: &Edge,
    right: &Edge,
    horizontal_edges: &mut BinaryHeap<Edge>,
) -> (Vec<Edge>, i32) {
    let mut popped = Vec::new();
    let mut matching = Vec::new();
    let mut height_found = None;
    while let Some(edge) = horizontal_edges.pop() {
        if let Some(height) = height_found {
            if edge.height < height {
                popped.push(edge);
                break;
            }
        }

        assert!(
            edge.height >= left.start_idx && edge.height >= right.start_idx,
            "edge: {:?}, left: {:?}, right: {:?}",
            edge,
            left,
            right
        );

        let range = left.height..=right.height;
        if range.contains(&edge.start_idx) || range.contains(&edge.end_idx) {
            matching.push(edge);
            height_found = Some(edge.height);
        } else {
            popped.push(edge);
        }
    }
    for edge in popped {
        horizontal_edges.push(edge);
    }

    (matching, height_found.expect("nothing found"))
}

// fn print_vert_edges(vert_edges: &HashSet<Edge>) {
//     println!("vert_edges:");
//     for edge in vert_edges {
//         println!("Vert_edge: {:?}", edge);
//     }
// }
//
// fn print_hor_edges(hor_edges: &mut BinaryHeap<Edge>) {
//     let mut popped = Vec::new();
//     while let Some(edge) = hor_edges.pop() {
//         println!("Hor_edge: {:?}", edge);
//         popped.push(edge);
//     }
//
//     for edge in popped {
//         hor_edges.push(edge);
//     }
// }

fn find_and_remove(height: i32, idx: i32, edges: &mut HashSet<Edge>) -> Edge {
    if let Some(edge) = find(height, idx, &edges) {
        let edge = *edge;
        edges.remove(&edge);
        edge
    } else {
        println!(
            "Did not found anything for {height}, {idx}, {}",
            edges.len()
        );
        for edge in edges.iter() {
            println!("having edge {:?}", edge);
        }
        panic!("meh.");
    }
}

fn find(height: i32, idx: i32, edges: &HashSet<Edge>) -> Option<&Edge> {
    edges
        .iter()
        .find(|edge| edge.height == idx && (edge.start_idx == height || edge.end_idx == height))
}

fn find_vert_edges(hor_edges: &[Edge], vertical_edges: &HashSet<Edge>) -> Vec<Edge> {
    chain(
        hor_edges
            .iter()
            .filter_map(|hor| find(hor.height, hor.start_idx, vertical_edges)),
        hor_edges
            .iter()
            .filter_map(|hor| find(hor.height, hor.end_idx, vertical_edges)),
    )
    .map(|val| *val)
    .collect()
}

fn solve(instructions: &[DigInstruction]) -> usize {
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
    dug.values()
        .filter(|value| !borders.contains(value))
        .count()
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

#[cfg(test)]
pub mod test18 {
    use crate::day18::{solve_p2, DigInstruction};

    #[test]
    fn test_basic() {
        let input = r#"R 4 (#000040)
U 4 (#000043)
L 4 (#000042)
D 4 (#000041)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(25, solve_p2(&instructions));
    }

    #[test]
    fn test_box() {
        let input = r#"R 1 (#000010)
D 3 (#000031)
R 2 (#000020)
U 3 (#000033)
R 1 (#000010)
U 4 (#000043)
L 4 (#000042)
D 4 (#000041)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(34, solve_p2(&instructions));
    }

    #[test]
    fn test_uneven_box() {
        let input = r#"R 1 (#000010)
D 3 (#000031)
R 2 (#000020)
U 2 (#000023)
R 1 (#000010)
U 5 (#000053)
L 4 (#000042)
D 4 (#000041)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(35, solve_p2(&instructions));
    }

    #[test]
    fn test_cut_box() {
        let input = r#"R 4 (#000040)
U 4 (#000043)
L 1 (#000012)
D 3 (#000031)
L 2 (#000022)
U 3 (#000033)
L 1 (#000012)
D 4 (#000041)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(22, solve_p2(&instructions));
    }

    #[test]
    fn test_add_below() {
        let input = r#"R 4 (#000040)
U 4 (#000043)
L 1 (#000012)
U 2 (#000023)
L 2 (#000022)
D 2 (#000021)
L 1 (#000012)
D 4 (#000041)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(31, solve_p2(&instructions));
    }

    #[test]
    fn test_tower() {
        let input = r#"R 1 (#000010)
D 3 (#000031)
R 1 (#000010)
U 3 (#000033)
R 1 (#000010)
D 4 (#000041)
R 1 (#000010)
U 5 (#000043)
R 1 (#000010)
U 1 (#000013)
L 5 (#000052)
D 1 (#000011)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(26, solve_p2(&instructions));
    }

    #[test]
    fn test_input() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        let instructions: Vec<DigInstruction> = input
            .lines()
            .map(|input| input.parse::<DigInstruction>().expect("or not"))
            .collect();
        assert_eq!(952408144115, solve_p2(&instructions));
    }
}
