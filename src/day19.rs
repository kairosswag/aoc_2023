use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::string::ParseError;
use std::time::Instant;

#[derive(Debug, FromStr, Clone)]
#[display("{name}{{{rules}}}")]
struct Workflow {
    name: String,
    rules: Rules,
}

#[derive(Debug, Clone)]
struct Rules {
    rules: Vec<Rule>,
    default: String,
}

impl FromStr for Rules {
    type Err = ParseError;

    fn from_str(rule_string: &str) -> Result<Self, Self::Err> {
        let mut rules_str: Vec<&str> = rule_string.split(',').collect();
        let mut rules = Vec::new();
        let default = rules_str
            .pop()
            .expect("should have at least the default rule")
            .parse()?;
        for rule_str in rules_str {
            let rule = rule_str.parse().expect("why cannot i use ? here???");
            rules.push(rule);
        }

        Ok(Rules { rules, default })
    }
}

#[derive(Display, FromStr, Clone, Debug)]
#[display("{variable}{operation}{value}:{success}")]
struct Rule {
    variable: char,
    #[from_str(regex = "[<>]")]
    operation: Operation,
    #[from_str(regex = "[0-9]+")]
    value: usize,
    success: String,
}

impl Rules {
    fn eval(&self, rating: &Rating) -> EvalResult {
        for rule in &self.rules {
            match rule.eval(rating) {
                Some(result) => return result,
                None => (),
            }
        }

        EvalResult::create_from_str(&self.default)
    }
}

impl Rule {
    fn eval(&self, rating: &Rating) -> Option<EvalResult> {
        let val = match self.variable {
            'a' => rating.a_rat,
            'm' => rating.m_rat,
            's' => rating.s_rat,
            'x' => rating.x_rat,
            _ => unreachable!("strange variant"),
        };

        let clears = match self.operation {
            Operation::GT => val > self.value,
            Operation::LT => val < self.value,
        };

        if clears {
            Some(EvalResult::create_from_str(&self.success))
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
enum EvalResult {
    Accept,
    Reject,
    NextRule(String),
}

impl EvalResult {
    fn create_from_str(success: &str) -> EvalResult {
        match success {
            "A" => EvalResult::Accept,
            "R" => EvalResult::Reject,
            rest => EvalResult::NextRule(String::from(rest)),
        }
    }

    fn is_final(&self) -> bool {
        match self {
            EvalResult::Accept | EvalResult::Reject => true,
            EvalResult::NextRule(_) => false,
        }
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
enum Operation {
    #[display("<")]
    LT,
    #[display(">")]
    GT,
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("{{x={x_rat},m={m_rat},a={a_rat},s={s_rat}}}")]
struct Rating {
    x_rat: usize,
    m_rat: usize,
    a_rat: usize,
    s_rat: usize,
}

impl Rating {
    fn sum_up(&self) -> usize {
        self.a_rat + self.m_rat + self.s_rat + self.x_rat
    }
}

pub fn run() {
    let file = fs::read_to_string("input/day19").expect("Could not open file.");
    let mut lines = file.lines();
    let workflows = lines
        .by_ref()
        .map_while(|line| line.parse::<Workflow>().ok())
        .map(|workflow| (workflow.name, workflow.rules))
        .collect::<HashMap<String, Rules>>();
    let ratings = lines
        .map(|line| line.parse::<Rating>().expect("no rating"))
        .collect::<Vec<Rating>>();

    let now = Instant::now();
    let (res_1, res_2) = (solve(&ratings, &workflows), solve_p2(&workflows));
    println!("Solutions took {} µs", now.elapsed().as_micros());
    println!("Day 19 Solution Part 1: {}", res_1);
    println!("Day 19 Solution Part 2: {}", res_2);
}

fn solve(ratings: &[Rating], workflows: &HashMap<String, Rules>) -> usize {
    let start_rule = workflows.get("in").expect("start should be available");
    let mut total = 0;
    for rating in ratings {
        let mut result = start_rule.eval(rating);
        while !result.is_final() {
            if let EvalResult::NextRule(next_workflow) = &result {
                let next = workflows
                    .get(next_workflow)
                    .expect("could not find next workflow");
                result = next.eval(rating);
            } else {
                unreachable!("huh?");
            }
        }
        if result == EvalResult::Accept {
            total += rating.sum_up();
        }
    }
    total
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct ValidRatings {
    x_rat: (usize, usize),
    m_rat: (usize, usize),
    a_rat: (usize, usize),
    s_rat: (usize, usize),
}

impl ValidRatings {
    fn initial() -> Self {
        let init_rating = (1, 4000);
        ValidRatings {
            x_rat: init_rating,
            m_rat: init_rating,
            a_rat: init_rating,
            s_rat: init_rating,
        }
    }

    fn modify(&self, pos: char, value: (usize, usize)) -> ValidRatings {
        let mut res = self.clone();
        match pos {
            'x' => res.x_rat = value,
            'm' => res.m_rat = value,
            'a' => res.a_rat = value,
            's' => res.s_rat = value,
            _ => unreachable!("nope variant"),
        }
        res
    }

    fn is_disjunct(&self, other: &ValidRatings) -> bool {
        let compare = |(s_min, s_max), (o_min, o_max)| {
            let self_fully_below = s_max < o_min;
            let other_fully_below = o_max < s_min;
            self_fully_below || other_fully_below
        };
        let x_disjunct = compare(self.x_rat, other.x_rat);
        let m_disjunct = compare(self.m_rat, other.m_rat);
        let a_disjunct = compare(self.a_rat, other.a_rat);
        let s_disjunct = compare(self.s_rat, other.s_rat);
        x_disjunct || m_disjunct || a_disjunct || s_disjunct
    }

    fn total(&self) -> usize {
        let x_delta = self.x_rat.1 - self.x_rat.0 + 1;
        let s_delta = self.s_rat.1 - self.s_rat.0 + 1;
        let a_delta = self.a_rat.1 - self.a_rat.0 + 1;
        let m_delta = self.m_rat.1 - self.m_rat.0 + 1;
        x_delta * s_delta * a_delta * m_delta
    }
}

impl Rules {
    fn expand(&self, valid_ratings: &ValidRatings) -> Vec<ExpansionResult> {
        let mut next = Some(valid_ratings.clone());
        let mut results = Vec::new();
        for rule in &self.rules {
            if let Some(to_apply) = next {
                let (when_applied, non_applied) = rule.split_apply(&to_apply);
                next = non_applied;

                match when_applied {
                    None => (),
                    Some(expansion_result) => results.push(expansion_result),
                }
            }
        }
        match (next, &self.default) {
            (None, _) => (),
            (Some(_), default) if default == "R" => (),
            (Some(rating), default) if default == "A" => {
                results.push(ExpansionResult::Accept(rating))
            }
            (Some(rating), default) => {
                results.push(ExpansionResult::Next(String::from(default), rating))
            }
        }
        results
    }
}

impl Rule {
    fn split_apply(
        &self,
        rating: &ValidRatings,
    ) -> (Option<ExpansionResult>, Option<ValidRatings>) {
        let val = match self.variable {
            'a' => rating.a_rat,
            'm' => rating.m_rat,
            's' => rating.s_rat,
            'x' => rating.x_rat,
            _ => unreachable!("strange variant"),
        };
        let split = match self.operation {
            Operation::GT => {
                if val.1 <= self.value {
                    (None, Some(val))
                } else if val.0 > self.value {
                    (Some(val), None)
                } else {
                    (Some((self.value + 1, val.1)), Some((val.0, self.value)))
                }
            }
            Operation::LT => {
                if val.0 >= self.value {
                    (None, Some(val))
                } else if val.1 < self.value {
                    (Some(val), None)
                } else {
                    (Some((val.0, self.value - 1)), Some((self.value, val.1)))
                }
            }
        };

        let exp_res = match (split.0, &self.success) {
            (None, _) => None,
            (Some(val), str) if str == "A" => {
                Some(ExpansionResult::Accept(rating.modify(self.variable, val)))
            }
            (Some(_), str) if str == "R" => None,
            (Some(val), str) => Some(ExpansionResult::Next(
                String::from(str),
                rating.modify(self.variable, val),
            )),
        };

        (
            exp_res,
            split.1.map(|val| rating.modify(self.variable, val)),
        )
    }
}

enum ExpansionResult {
    Accept(ValidRatings),
    Next(String, ValidRatings),
}

fn solve_p2(workflows: &HashMap<String, Rules>) -> usize {
    let mut candidates = Vec::new();
    candidates.push((String::from("in"), ValidRatings::initial()));
    let mut accepted = Vec::new();

    while let Some(candidate) = candidates.pop() {
        let rules = workflows.get(&candidate.0).unwrap();
        let results = rules.expand(&candidate.1);
        for result in results {
            match result {
                ExpansionResult::Accept(accepted_rating) => accepted.push(accepted_rating),
                ExpansionResult::Next(name, ratings) => candidates.push((name, ratings)),
            }
        }
    }
    for accept in &accepted {
        for accept_b in &accepted {
            if accept != accept_b && !accept.is_disjunct(accept_b) {
                println!("not disjunct {:?} {:?}", accept, accept_b);
            }
        }
    }
    accepted.iter().map(ValidRatings::total).sum()
}

#[test]
fn test() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}
"#;
    let workflows = input
        .lines()
        .map(|line| line.parse::<Workflow>().expect("could not parse workflow"))
        .map(|workflow| (workflow.name, workflow.rules))
        .collect();
    assert_eq!(167409079868000, solve_p2(&workflows));
}

#[test]
fn test_eq() {
    assert!(!ValidRatings::initial().is_disjunct(&ValidRatings::initial()))
}
