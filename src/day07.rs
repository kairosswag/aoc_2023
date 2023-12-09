use std::cmp::Ordering;
use std::fs;

use itertools::Itertools;

#[derive(Clone, Copy)]
struct CamelBid {
    hand: Hand,
    bid: u32,
}

#[derive(Clone, Copy, Debug)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
}

impl Hand {
    fn from_cards(cards: [char; 5]) -> Hand {
        use HandType::*;
        let mut ccards = cards.clone();
        ccards.sort();
        let hand_type = match ccards
            .group_by(|a, b| *a == *b)
            .map(|grp| grp.len())
            .sorted()
            .pad_using(5, |_| 0)
            .tuples::<(_, _, _, _, _)>()
            .next()
            .unwrap()
        {
            (5, 0, 0, 0, 0) => FiveOfAKind,
            (1, 4, 0, 0, 0) => FourOfAKind,
            (2, 3, 0, 0, 0) => FullHouse,
            (1, 1, 3, 0, 0) => ThreeOfAKind,
            (1, 2, 2, 0, 0) => TwoPair,
            (1, 1, 1, 2, 0) => OnePair,
            (1, 1, 1, 1, 1) => HighCard,
            _ => unreachable!("whelp"),
        };
        Hand { cards, hand_type }
    }

    fn from_cards_extra_rule(cards: [char; 5]) -> Hand {
        use HandType::*;
        let binding = cards.into_iter().filter(|card| *card != '!').sorted();
        let ccards: &[char] = binding.as_ref();
        let hand_type = match ccards
            .group_by(|a, b| *a == *b)
            .map(|grp| grp.len())
            .sorted()
            .pad_using(5, |_| 0)
            .tuples::<(_, _, _, _, _)>()
            .next()
            .unwrap()
        {
            (_, 0, 0, 0, 0) => FiveOfAKind,
            (1, 4, 0, 0, 0) | (1, 3, 0, 0, 0) | (1, 2, 0, 0, 0) | (1, 1, 0, 0, 0) => FourOfAKind,
            (2, 3, 0, 0, 0) | (2, 2, 0, 0, 0) => FullHouse,
            (1, 1, 3, 0, 0) | (1, 1, 2, 0, 0) | (1, 1, 1, 0, 0) => ThreeOfAKind,
            (1, 2, 2, 0, 0) => TwoPair,
            (1, 1, 1, 2, 0) | (1, 1, 1, 1, 0) => OnePair,
            (1, 1, 1, 1, 1) => HighCard,
            _ => unreachable!("whelp"),
        };
        Hand { cards, hand_type }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn run() {
    let res_1: u32 = fs::read_to_string("input/day07")
        .expect("Could not open file.")
        .lines()
        .map(|line| parse_hand(line))
        .sorted_by(camel_comparator)
        .zip(1..)
        .map(|(bid, rank)| rank * bid.bid)
        .sum();

    let res_2: u32 = fs::read_to_string("input/day07")
        .expect("Could not open file.")
        .lines()
        .map(|line| parse_hand_extra_rule(line))
        .sorted_by(camel_comparator)
        .zip(1..)
        .map(|(bid, rank)| rank * bid.bid)
        .sum();
    let res = (res_1, res_2);
    println!("Day 07 Solution Part 1: {}", res.0);
    println!("Day 07 Solution Part 2: {}", res.1);
}

fn parse_hand(line: &str) -> CamelBid {
    let (hand, bid) = line
        .split_whitespace()
        .tuples()
        .next()
        .expect("parsing bleh");

    let cards: [char; 5] = hand
        .chars()
        .take(5)
        .map(|card| match card {
            'A' => 'Z',
            'K' => 'X',
            'T' => 'B',
            rest => rest,
        })
        .collect::<Vec<char>>()
        .try_into()
        .expect("no clue what this is doing");

    let hand = Hand::from_cards(cards);
    let bid = bid.parse::<u32>().expect("bid you not");
    CamelBid { hand, bid }
}

fn parse_hand_extra_rule(line: &str) -> CamelBid {
    let (hand, bid) = line
        .split_whitespace()
        .tuples()
        .next()
        .expect("parsing bleh");

    let cards: [char; 5] = hand
        .chars()
        .take(5)
        .map(|card| match card {
            'A' => 'Z',
            'K' => 'X',
            'T' => 'B',
            'J' => '!',
            rest => rest,
        })
        .collect::<Vec<char>>()
        .try_into()
        .expect("no clue what this is doing");

    let hand = Hand::from_cards_extra_rule(cards);
    let bid = bid.parse::<u32>().expect("bid you not");
    CamelBid { hand, bid }
}

fn camel_comparator(a: &CamelBid, b: &CamelBid) -> Ordering {
    match a.hand.hand_type.cmp(&b.hand.hand_type) {
        Ordering::Equal => hand_comparator(&a.hand.cards, &b.hand.cards),
        val => val,
    }
}

fn hand_comparator(a: &[char; 5], b: &[char; 5]) -> Ordering {
    for idx in 0..5 {
        match a[idx].cmp(&b[idx]) {
            Ordering::Equal => continue,
            gt_lt => return gt_lt,
        }
    }
    unreachable!("wait what?")
}

#[cfg(test)]
mod test {
    use crate::day07::Hand;

    #[test]
    fn test() {
        println!(
            "hand: {:?}",
            Hand::from_cards_extra_rule("32T!!".chars().collect::<Vec<char>>().try_into().unwrap())
        )
    }
}
