use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::str::Lines;
use std::time::Instant;

pub fn run() {
    let file = fs::read_to_string("input/day11").expect("Could not open file.");

    let universe = parse_universe(file.lines(), 1);
    let res_1 = solve(&universe);

    let universe = parse_universe(file.lines(), 1000000);
    let res_2 = solve(&universe);

    println!("Day 11 Solution Part 1: {}", res_1);
    println!("Day 11 Solution Part 2: {}", res_2);
}

fn solve(universe: &HashSet<(usize, usize)>) -> usize {
    universe
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| calc_distance(g1, g2))
        .sum()
}

fn calc_distance((g1_x, g1_y): &(usize, usize), (g2_x, g2_y): &(usize, usize)) -> usize {
    if g1_x == g2_x && g1_y == g2_y {
        0
    } else {
        (g1_x.max(g2_x) - g1_x.min(g2_x)) + (g1_y.max(g2_y) - g1_y.min(g2_y))
    }
}

fn parse_universe(lines: Lines, expansion_factor: usize) -> HashSet<(usize, usize)> {
    let mut empty_x = vec![true; 150];
    let mut empty_y = vec![true; 150];
    let mut universe = HashSet::new();
    for (y_val, line) in lines.enumerate() {
        for (x_val, char) in line.char_indices() {
            match char {
                '.' => (),
                '#' => {
                    universe.insert((x_val, y_val));
                    empty_x[x_val] = false;
                    empty_y[y_val] = false;
                }
                _ => unreachable!("other characters? O_o"),
            };
        }
    }
    let mut real_universe = HashSet::new();
    for (galaxy_xpos, galaxy_ypos) in universe {
        let count_spaces = |spaaaace: &[bool], pos: usize| -> usize {
            spaaaace[0..pos].iter().filter(|val| **val).count() * (expansion_factor - 1).max(1)
        };

        let real_xpos = galaxy_xpos + count_spaces(&empty_x, galaxy_xpos);
        let real_ypos = galaxy_ypos + count_spaces(&empty_y, galaxy_ypos);

        real_universe.insert((real_xpos, real_ypos));
    }

    real_universe
}

#[test]
fn test() {
    let test_input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    let universe = parse_universe(test_input.lines(), 1);
    let test_0 = solve(&universe);

    let universe = parse_universe(test_input.lines(), 10);
    let test_1 = solve(&universe);

    let universe = parse_universe(test_input.lines(), 100);
    let test_2 = solve(&universe);

    assert_eq!(374, test_0);
    assert_eq!(1030, test_1);
    assert_eq!(8410, test_2);
}
