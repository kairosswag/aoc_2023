use std::fs;
use std::str::Lines;
use std::time::Instant;

pub fn run() {
    let file = fs::read_to_string("input/day13").expect("Could not open file.");
    let now = Instant::now();
    let (res_1, res_2) = solve(file.lines());
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 12 Solution Part 1: {}", res_1);
    println!("Day 12 Solution Part 2: {}", res_2);
}

struct Grid {
    rows: Vec<usize>,
    columns: Vec<usize>,
}

fn solve(mut lines: Lines) -> (usize, usize) {
    let mut total = 0;
    while let Some(grid) = parse_grid(&mut lines) {
        let row_reflections = determine_reflections(&grid.rows);
        let col_reflections = determine_reflections(&grid.columns);
        total += col_reflections.iter().sum::<usize>();
        total += row_reflections.iter().map(|val| val * 100).sum::<usize>();
    }

    (total, 5)
}

fn determine_reflections(val: &[usize]) -> Vec<usize> {
    let mut found = Vec::new();
    for pivot in 1..val.len() {
        let steps = pivot.max(val.len() - pivot);
        if (0..=steps).all(|idx| {
            matches(
                &val.get((pivot - 1).wrapping_sub(idx)),
                &val.get(pivot + idx),
            )
        }) {
            found.push(pivot);
        }
    }
    found
}

fn matches(a: &Option<&usize>, b: &Option<&usize>) -> bool {
    match (a, b) {
        (None, _) => true,
        (_, None) => true,
        (Some(a), Some(b)) => *a == *b,
    }
}

fn parse_grid(lines: &mut Lines) -> Option<Grid> {
    let mut rows = Vec::new();
    let mut columns = vec![0; 25];
    let mut col_num = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        col_num = line.len();

        let mut row_val = 0;

        for (y_pos, symbol) in line.char_indices() {
            let mut col_val = columns[y_pos];
            col_val = col_val << 1;
            row_val = row_val << 1;
            let symbol_value = match symbol {
                '#' => 1,
                '.' => 0,
                _ => unreachable!("strange symbol"),
            };
            col_val += symbol_value;
            row_val += symbol_value;

            columns[y_pos] = col_val;
        }

        rows.push(row_val);
    }
    columns.truncate(col_num);
    if rows.len() == 0 {
        None
    } else {
        Some(Grid { rows, columns })
    }
}

#[test]
fn test_1() {
    let test = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
    assert_eq!((5, 5), solve(test.lines()));
}

#[test]
fn test_2() {
    let test = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    assert_eq!((400, 5), solve(test.lines()));
}
