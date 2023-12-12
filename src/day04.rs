use std::collections::HashSet;
use std::fs;
use std::iter::zip;
use std::str::Lines;

struct Card {
    line_no: u32,
    winning: HashSet<u32>,
    yours: HashSet<u32>,
}

impl Card {
    fn count_winning(&self) -> usize {
        self.yours.intersection(&self.winning).count()
    }

    fn count_winning_points(&self) -> u32 {
        let res_count = self.count_winning();
        if res_count > 0 {
            u32::pow(2, (res_count - 1) as u32)
        } else {
            0
        }
    }
}

pub fn run() {
    let res = crate::day04::solve(
        fs::read_to_string("input/day04")
            .expect("Could not open file.")
            .lines(),
    );
    println!("Day 04 Solution Part 1: {}", res.0);
    println!("Day 04 Solution Part 2: {}", res.1);
}

fn solve(lines: Lines<'_>) -> (u32, u32) {
    let cards = zip(1.., lines.clone()).map(|(line_no, line)| parse_card(line, line_no));

    let cards: Vec<Card> = cards.collect();
    let res_p1 = cards.iter().map(|card| card.count_winning_points()).sum();

    let mut won = vec![1; cards.len()];

    for card in cards {
        let winning = card.count_winning() as u32;

        for card_won in (card.line_no + 1)..(card.line_no + 1 + winning) {
            let multi = won[to_idx(card.line_no)];
            won[to_idx(card_won)] += multi;
        }
    }

    (res_p1, won.iter().sum())
}

fn to_idx(line_no: u32) -> usize {
    (line_no - 1) as usize
}
fn parse_card(line: &str, line_no: u32) -> Card {
    let mut split_results = line.split(':').nth(1).expect("wrong parsing").split("|");
    let winning = parse_numbers(split_results.nth(0).expect("should be a value"));
    let yours = parse_numbers(split_results.nth(0).expect("should be a value"));

    Card {
        line_no,
        winning,
        yours,
    }
}

fn parse_numbers(input: &str) -> HashSet<u32> {
    let mut parsed_set = HashSet::new();
    for hopeful_number in input.split_whitespace() {
        let hopeful_number = hopeful_number.trim();
        parsed_set.insert(
            hopeful_number
                .parse::<u32>()
                .expect("Not a number after all"),
        );
    }
    parsed_set
}
