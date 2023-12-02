use std::fs;
use std::str::Lines;

use parse_display::*;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Game {val}")]
pub struct GameNumber {
    val: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn new() -> Draw {
        Draw { red: 0, green: 0, blue: 0 }
    }
    fn new_max() -> Draw {
        Draw { red: u32::MAX, green: u32::MAX, blue: u32::MAX }
    }

    fn fold(&mut self, other: &Draw) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }

    fn minimize(&self, other: &Draw) -> Draw {
        let red = u32::max(self.red, other.red);
        let green = u32::max(self.green, other.green);
        let blue = u32::max(self.blue, other.blue);
        Draw { red, green, blue }
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{0}")]
pub enum DrawVariant {
    #[display("{0} red")]
    Red(u32),
    #[display("{0} green")]
    Green(u32),
    #[display("{0} blue")]
    Blue(u32),
}

impl DrawVariant {
    fn increment(self, mut draw: Draw) -> Draw {
        match self {
            DrawVariant::Red(val) => { draw.red += val }
            DrawVariant::Green(val) => { draw.green += val }
            DrawVariant::Blue(val) => { draw.blue += val }
        }
        draw
    }
}


pub fn run() {
    let res = solve(fs::read_to_string("input/day02")
        .expect("Could not open file.").lines());


    println!("Day 02 Solution Part 1: {}", res.0);
    println!("Day 02 Solution Part 2: {}", res.1);
}

pub fn solve(val: Lines<'_>) -> (u32, u32) {
    let games: Vec<(GameNumber, Vec<Draw>)> = val.map(|line| {
        let (game, draws) = line.trim().split_once(':').unwrap();
        let number = game.parse::<GameNumber>().expect(&format!("Could not be parse number {}", game));
        let draws = parse_draws(draws);
        (number, draws)
    }).collect();

    let res_p1: u32 = games.iter().map(|(number, draws)| {
        let valid = determine_valid_p1(draws);

        (number, valid)
    }).filter(|(_n, success)| *success).map(|(number, _s)| number.val).sum();

    let res_p2: u32 = games.iter()
        .map(|(_n, draws)| draws)
        .map(|draws| draws.iter().fold(Draw::new(), |accu, draw| accu.minimize(draw)))
        .map(|min_draw| min_draw.red * min_draw.green * min_draw.blue)
        .sum();

    (res_p1, res_p2)
}

fn determine_valid_p1(draws: &[Draw]) -> bool {
    for total in draws {
        let valid = total.red <= 12 && total.green <= 13 && total.blue <= 14;
        if !valid {
            return false;
        }
    }
    true
}

pub fn parse_draws(draws: &str) -> Vec<Draw> {
    draws.split(';').map(|split_semi| {
        split_semi.split(',').map(|split| split.trim())
            .map(|val| val.parse::<DrawVariant>().expect("could not get variant"))
            .fold(Draw::new(), |accu, val| val.increment(accu))
    }).collect()
}

#[cfg(test)]
mod day02_test {
    use crate::day02::{Draw, parse_draws, solve};

    #[test]
    pub fn test_p1() {
        let lines =
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.lines();
        solve(lines);
    }

    #[test]
    pub fn test_parse() {
        let res = parse_draws(&"8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        println!("{:?}", res);
        let total = res.iter().fold(Draw::new(), |mut accu, other| {
            accu.fold(other);
            accu
        });
        println!("{:?}", total);
    }
}
