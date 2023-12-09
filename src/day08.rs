use itertools::Itertools;
use num::integer::lcm;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::fs;
use std::str::{FromStr, Lines};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Node(char, char, char);

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, c) = s.chars().next_tuple().expect("meh");
        Ok(Node(a, b, c))
    }
}

#[derive(FromStr)]
#[display("{nav_start} = ({nav_l}, {nav_r})")]
struct NavLine {
    nav_start: Node,
    nav_l: Node,
    nav_r: Node,
}

pub fn run() {
    let file = fs::read_to_string("input/day08").expect("Could not open file.");
    let mut lines = file.lines();
    let path = lines.next().unwrap();

    let (l_map, r_map) = parse_map(lines);

    let res_1 = count_steps(path, &l_map, &r_map);
    let res_2 = count_steps_simultaneously(path, &l_map, &r_map);

    let res = (res_1, res_2);
    println!("Day 08 Solution Part 1: {}", res.0);
    println!("Day 08 Solution Part 2: {}", res.1);
}

fn count_steps_simultaneously(
    path: &str,
    l_map: &HashMap<Node, Node>,
    r_map: &HashMap<Node, Node>,
) -> u64 {
    l_map
        .keys()
        .filter(|key| match key {
            Node(_, _, 'A') => true,
            _ => false,
        })
        .map(|node| count_steps_starting(node, path, l_map, r_map, &compare_any_end))
        .map(|val| val as u64)
        .reduce(lcm)
        .expect("should be fine")
}

fn count_steps(path: &str, l_map: &HashMap<Node, Node>, r_map: &HashMap<Node, Node>) -> u32 {
    let start_point = Node('A', 'A', 'A');
    count_steps_starting(&start_point, path, l_map, r_map, &compare_end)
}

fn compare_end(node: &Node) -> bool {
    match node {
        Node('Z', 'Z', 'Z') => true,
        _ => false,
    }
}

fn compare_any_end(node: &Node) -> bool {
    match node {
        Node(_, _, 'Z') => true,
        _ => false,
    }
}

fn count_steps_starting(
    start_point: &Node,
    path: &str,
    l_map: &HashMap<Node, Node>,
    r_map: &HashMap<Node, Node>,
    com_fn: &dyn Fn(&Node) -> bool,
) -> u32 {
    let mut curr = start_point;
    let mut steps = 0;

    loop {
        for instr in path.chars() {
            steps += 1;

            let map_to_use = match instr {
                'L' => l_map,
                'R' => r_map,
                _ => unreachable!(),
            };

            let next = map_to_use
                .get(&curr)
                .expect("Map does not contain curr node!");

            if com_fn(next) {
                return steps;
            }
            curr = next;
        }
    }
}

fn parse_map(lines: Lines) -> (HashMap<Node, Node>, HashMap<Node, Node>) {
    let mut l_map = HashMap::new();
    let mut r_map = HashMap::new();

    for line in lines.skip(1) {
        let nav_line = line.parse::<NavLine>().expect("could not parse line");
        l_map.insert(nav_line.nav_start, nav_line.nav_l);
        r_map.insert(nav_line.nav_start, nav_line.nav_r);
    }

    (l_map, r_map)
}
