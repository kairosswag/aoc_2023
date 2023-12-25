use crate::day20::Pulse::{HIGH, LOW};
use crate::day20::Type::{Conjunction, FlipFlop};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::Lines;
use std::time::Instant;

const START: &str = "broadcaster";
pub fn run() {
    let file = fs::read_to_string("input/day20").expect("Could not open file.");
    let modules = parse(file.lines());

    let now = Instant::now();
    let (res_1, res_2) = (solve(&modules), solve_p2(&modules));
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 20 Solution Part 1: {}", res_1);
    println!("Day 20 Solution Part 2: {}", res_2);
}

fn parse(lines: Lines) -> HashMap<&str, (Type, Vec<&str>)> {
    let mut modules = HashMap::new();
    for line in lines {
        parse_module(line, &mut modules);
    }
    modules
}

fn solve_p2(modules: &HashMap<&str, (Type, Vec<&str>)>) -> usize {
    // Asertion: rx is always fed by one conjunction, conjoining n conjunctions
    // the joined conjunctions are independent
    // we need to find out the cycle time of each conjunction (they all need to fire high in the same turn)
    let mut conjunction_state = extract_conjunction_states(modules);
    let mut flip_flop_state: HashMap<&str, bool> = HashMap::new();
    let reverse_modules = modules
        .iter()
        .flat_map(|(name, (typ, following))| {
            following.iter().map(|follower| (*follower, (*name, *typ)))
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&str, Vec<(&str, Type)>>, (key, value)| {
                let entry = acc
                    .entry(key)
                    .and_modify(|values| values.push(value))
                    .or_insert(vec![value]);

                acc
            },
        );

    let mut flip_flop_groups = HashMap::new();

    for (idx, (name, (typ, others))) in modules.iter().enumerate() {
        if *typ == FlipFlop {
            put_or_follow(*name, idx, &mut flip_flop_groups, modules);
        }
    }

    let mut node_conjunctions: HashMap<&str, Option<usize>> = flip_flop_groups
        .iter()
        .map(|(_ff_name, conj_name)| *conj_name)
        .unique()
        .map(|val| (val, None))
        .collect();

    let mut queue = VecDeque::new();

    for press in 1.. {
        queue.push_front((START, Pulse::LOW, START));
        while let Some((name, pulse, source)) = queue.pop_back() {
            let module = modules.get(name);
            let module = if let Some(module) = module {
                module
            } else {
                assert_eq!("rx", name);
                continue;
            };
            match module {
                (Type::Start, targets) => {
                    for target in targets {
                        queue.push_front((*target, pulse, name))
                    }
                }
                (Type::FlipFlop, targets) => {
                    if pulse != HIGH {
                        let state = *flip_flop_state.get(name).unwrap_or(&false);

                        let pulse = if state { LOW } else { HIGH };
                        flip_flop_state.insert(name, !state);

                        for target in targets {
                            queue.push_front((*target, pulse, name));
                        }
                    }
                }
                (Type::Conjunction, targets) => {
                    let pulse =
                        update_conjunction_and_trigger(name, source, pulse, &mut conjunction_state);
                    for target in targets {
                        queue.push_front((*target, pulse, name));
                    }
                }
            }
        }
        if press % 1_000_000 == 0 {
            println!("press {press}");
        }
        let mut updated = false;
        // after each press: look what the state of the relevant conjunctions is
        for (conjunction, value) in node_conjunctions.iter_mut() {
            if conjunction_state
                .get(*conjunction)
                .unwrap()
                .iter()
                .all(|state| state.1 == HIGH)
            {
                if let Some(value) = value {
                    if press % *value != 0 {
                        println!("invalid value for {conjunction} found at {press}, already got {value}, mod is {}", press % *value);
                    } else {
                        println!("found repetition of {conjunction} at {value}")
                    }
                } else {
                    println!("found initial {conjunction} at {press}");
                    *value = Some(press);
                    updated = true;
                }
            }
        }
        if updated && node_conjunctions.iter().all(|val| val.1.is_some()) {
            break;
        }
    }

    node_conjunctions
        .iter()
        .for_each(|val| println!("value: {:?}", val));

    5
}

fn put_or_follow<'a>(
    name: &'a str,
    idx: usize,
    groups: &mut HashMap<&'a str, &'a str>,
    modules: &HashMap<&str, (Type, Vec<&'a str>)>,
) -> Option<&'a str> {
    let mut conj_name = None;
    if let Some(group) = groups.get(&name) {
        conj_name = Some(*group);
    } else {
        if let Some((FlipFlop, following)) = modules.get(&name) {
            for next in following {
                let new_name = put_or_follow(*next, idx, groups, modules);
                if new_name.is_some() {
                    conj_name = new_name;
                }
            }
            groups.insert(name, conj_name.expect("did not get group"));
        } else if let Some((Conjunction, _)) = modules.get(&name) {
            conj_name = Some(&name)
        }
    }
    conj_name
}

fn solve(modules: &HashMap<&str, (Type, Vec<&str>)>) -> usize {
    let mut conjunction_state = extract_conjunction_states(modules);
    let mut flip_flop_state: HashMap<&str, bool> = HashMap::new();

    let mut queue = VecDeque::new();
    let mut tally = Tally::new();
    let mut presses_needed = None;

    for press in 1..=1000 {
        tally.count(LOW);
        queue.push_front((START, Pulse::LOW, START));
        while let Some((name, pulse, source)) = queue.pop_back() {
            let module = modules.get(name);
            let module = if let Some(module) = module {
                module
            } else {
                assert_eq!("rx", name);
                if pulse == LOW && presses_needed.is_none() {
                    presses_needed = Some(press);
                }
                continue;
            };
            match module {
                (Type::Start, targets) => {
                    for target in targets {
                        tally.count(pulse);
                        queue.push_front((*target, pulse, name))
                    }
                }
                (Type::FlipFlop, targets) => {
                    if pulse != HIGH {
                        let state = *flip_flop_state.get(name).unwrap_or(&false);

                        let pulse = if state { LOW } else { HIGH };
                        flip_flop_state.insert(name, !state);

                        for target in targets {
                            tally.count(pulse);
                            queue.push_front((*target, pulse, name));
                        }
                    }
                }
                (Type::Conjunction, targets) => {
                    let pulse =
                        update_conjunction_and_trigger(name, source, pulse, &mut conjunction_state);
                    for target in targets {
                        tally.count(pulse);
                        queue.push_front((*target, pulse, name));
                    }
                }
            }
        }
    }
    tally.low_pulses * tally.high_pulses
}

fn extract_conjunction_states<'a>(
    modules: &HashMap<&'a str, (Type, Vec<&'a str>)>,
) -> HashMap<&'a str, Vec<(&'a str, Pulse)>> {
    let mut conjunction_state: HashMap<_, _> = modules
        .iter()
        .filter(|(_, (typ, _))| *typ == Conjunction)
        .map(|(key, _)| (*key, Vec::new()))
        .collect();
    for (name, (_, targets)) in modules {
        for target in targets {
            if conjunction_state.contains_key(target) {
                let entry = conjunction_state.entry(target);
                entry.and_modify(|val| val.push((*name, LOW)));
            }
        }
    }
    conjunction_state
}

fn update_conjunction_and_trigger(
    conj_name: &str,
    trigger_name: &str,
    pulse: Pulse,
    conj_state: &mut HashMap<&str, Vec<(&str, Pulse)>>,
) -> Pulse {
    let states = conj_state
        .get_mut(conj_name)
        .expect("must not have built the map properly");
    let mut all_high = true;
    for val in states.iter_mut() {
        if trigger_name == val.0 {
            val.1 = pulse;
        }
        if val.1 == LOW {
            all_high = false;
        }
    }

    if all_high {
        LOW
    } else {
        HIGH
    }
}

struct Tally {
    low_pulses: usize,
    high_pulses: usize,
}

impl Tally {
    fn new() -> Self {
        Tally {
            low_pulses: 0,
            high_pulses: 0,
        }
    }
    fn count(&mut self, pulse: Pulse) {
        match pulse {
            Pulse::LOW => self.low_pulses += 1,
            Pulse::HIGH => self.high_pulses += 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pulse {
    LOW,
    HIGH,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Type {
    Start,
    FlipFlop,
    Conjunction,
}

fn parse_module<'map>(line: &'map str, modules: &mut HashMap<&'map str, (Type, Vec<&'map str>)>) {
    let mut line = line.split("->");
    let name = line.next().expect("could not parse name").trim();
    let (typ, name) = if name.starts_with("%") {
        (FlipFlop, &name[1..])
    } else if name.starts_with("&") {
        (Conjunction, &name[1..])
    } else {
        (Type::Start, START)
    };

    let targets = line
        .next()
        .expect("could not parse targets")
        .split(',')
        .map(|val| val.trim())
        .collect();

    modules.insert(name, (typ, targets));
}

#[test]
fn test_example() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    let modules = parse(input.lines());
    assert_eq!(32000000, solve(&modules));
}
