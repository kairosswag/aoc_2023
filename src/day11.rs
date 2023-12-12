use std::collections::HashSet;
use std::fs;
use std::str::Lines;

pub fn run() {
    let file = fs::read_to_string("input/day11").expect("Could not open file.");

    let universe = parse_universe(file.lines());
    let res_1: usize = universe
        .iter()
        .map(|galaxy_1| {
            universe
                .iter()
                .map(|galaxy_2| calc_distance(galaxy_1, galaxy_2))
                .sum::<usize>()
        })
        .sum();
    println!("Day 11 Solution Part 1: {}", res_1 / 2);
    println!("Day 11 Solution Part 2: {}", 5);
}

fn calc_distance((g1_x, g1_y): &(usize, usize), (g2_x, g2_y): &(usize, usize)) -> usize {
    if g1_x == g2_x && g1_y == g2_y {
        0
    } else {
        g1_x.max(g2_x) - g1_x.min(g2_x) + g1_y.max(g2_y) - g1_y.min(g2_y)
    }
}

fn parse_universe(lines: Lines) -> HashSet<(usize, usize)> {
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
            spaaaace[0..pos].iter().filter(|val| **val).count()
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

    let universe = parse_universe(test_input.lines());
    let res_1: usize = universe
        .iter()
        .map(|galaxy_1| {
            universe
                .iter()
                .map(|galaxy_2| calc_distance(galaxy_1, galaxy_2))
                .sum::<usize>()
        })
        .sum();

    assert_eq!(374, res_1 / 2);
}
