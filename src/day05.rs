use std::collections::VecDeque;
use std::fs;
use std::str::Lines;

use itertools::Itertools;

use crate::day05::State::{Before, Beyond, Within};

#[derive(Copy, Clone, Debug)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
enum State {
    Before,
    Within,
    Beyond,
}

struct Almanac {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

impl Almanac {
    fn sort_by_source(&mut self) {
        self.seeds.sort();
        sort_by_source_id(&mut self.seeds_to_soil);
        sort_by_source_id(&mut self.soil_to_fertilizer);
        sort_by_source_id(&mut self.fertilizer_to_water);
        sort_by_source_id(&mut self.water_to_light);
        sort_by_source_id(&mut self.light_to_temperature);
        sort_by_source_id(&mut self.temperature_to_humidity);
        sort_by_source_id(&mut self.humidity_to_location);
    }
}

pub fn run() {
    let mut almanac = crate::day05::parse(
        fs::read_to_string("input/day05")
            .expect("Could not open file.")
            .lines(),
    );
    let orig_seeds = almanac.seeds.clone();
    almanac.sort_by_source();
    let res_p1 = solve_p1(&almanac);
    let res_p2 = solve_p2(&almanac, orig_seeds);

    let res = (res_p1, res_p2);
    println!("Day 05 Solution Part 1: {}", res.0);
    println!("Day 05 Solution Part 2: {}", res.1);
}

fn solve_p1(almanac: &Almanac) -> u64 {
    let mut soils = get_destinations(&almanac.seeds, &almanac.seeds_to_soil);
    soils.sort();
    let mut fertilizer = get_destinations(&soils, &almanac.soil_to_fertilizer);
    fertilizer.sort();
    let mut water = get_destinations(&fertilizer, &almanac.fertilizer_to_water);
    water.sort();
    let mut light = get_destinations(&water, &almanac.water_to_light);
    light.sort();
    let mut temperature = get_destinations(&light, &almanac.light_to_temperature);
    temperature.sort();
    let mut humidity = get_destinations(&temperature, &almanac.temperature_to_humidity);
    humidity.sort();
    let mut location = get_destinations(&humidity, &almanac.humidity_to_location);
    location.sort();

    location[0]
}

fn solve_p2(almanac: &Almanac, original_seeds: Vec<u64>) -> u64 {
    let mut ranges: VecDeque<_> = original_seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(start, range)| (*start as u64, start + range))
        .collect();
    ranges.make_contiguous().sort_by(|t1, t2| t1.0.cmp(&t2.0));
    let ranges = extend_ranges(ranges, &almanac.seeds_to_soil);
    let ranges = extend_ranges(ranges, &almanac.soil_to_fertilizer);
    let ranges = extend_ranges(ranges, &almanac.fertilizer_to_water);
    let light = extend_ranges(ranges, &almanac.water_to_light);
    let temperature = extend_ranges(light, &almanac.light_to_temperature);
    let ranges = extend_ranges(temperature, &almanac.temperature_to_humidity);
    let ranges = extend_ranges(ranges, &almanac.humidity_to_location);

    ranges[0].0
}

fn extend_ranges(mut sources: VecDeque<(u64, u64)>, ranges: &[Range]) -> VecDeque<(u64, u64)> {
    let mut dest_ranges = VecDeque::new();

    'outer: while let Some((source_start, source_end)) = sources.pop_front() {
        for range in ranges {
            let range_start = range.source_range_start;
            let range_end = range_start + range.range_length - 1;

            let start = point_in_relation(source_start, range_start, range_end);
            let end = point_in_relation(source_end, range_start, range_end);

            use State::*;
            match (start, end) {
                (Before, Before) => (),
                (Before, Within) => {
                    dest_ranges.push_back((source_start, range_start - 1));
                    let diff = source_end - range_start;
                    dest_ranges.push_back((
                        range.destination_range_start,
                        range.destination_range_start + diff,
                    ));
                    continue 'outer;
                }
                (Before, Beyond) => {
                    dest_ranges.push_back((source_start, range_start - 1));
                    dest_ranges.push_back((
                        range.destination_range_start,
                        range.destination_range_start + range.range_length,
                    ));
                    sources.push_front((range_end + 1, source_end));
                    continue 'outer;
                }
                (Within, Before) => unreachable!(),
                (Within, Within) => {
                    // source starts above, end within
                    let diff_start = source_start - range_start;
                    let diff_end = source_end - range_start;
                    dest_ranges.push_back((
                        range.destination_range_start + diff_start,
                        range.destination_range_start + diff_end,
                    ));
                    continue 'outer;
                }
                (Within, Beyond) => {
                    let diff = source_start - range_start;
                    dest_ranges.push_back((
                        range.destination_range_start + diff,
                        range.destination_range_start + range.range_length,
                    ));
                    sources.push_front((range_end + 1, source_end));
                    continue 'outer;
                }
                (Beyond, Before) => unreachable!(),
                (Beyond, Within) => unreachable!(),
                (Beyond, Beyond) => (),
            }
        }
        dest_ranges.push_back((source_start, source_end));
    }

    dest_ranges
        .make_contiguous()
        .sort_by(|(s1, _), (s2, _)| s1.cmp(&s2));

    let mut consolidated = VecDeque::new();

    let mut curr = dest_ranges.pop_back().unwrap();
    while let Some(range) = dest_ranges.pop_back() {
        let higher_start = curr.0;
        let higher_end = curr.1;

        let lower_start = range.0;
        let lower_end = range.1;

        if lower_end >= higher_start {
            curr = (lower_start, higher_end.max(lower_end))
        } else {
            consolidated.push_front(curr);
            curr = range;
        }
    }
    consolidated.push_front(curr);

    consolidated
}

fn point_in_relation(point: u64, start_idx: u64, end_idx: u64) -> State {
    if start_idx > point {
        Before
    } else if start_idx <= point && end_idx >= point {
        Within
    } else if end_idx < point {
        Beyond
    } else {
        panic!("this should not be viable!");
    }
}

fn get_destinations(sources: &[u64], ranges: &[Range]) -> Vec<u64> {
    let mut lower_idx = 0;
    let mut destinations = Vec::new();
    'outer: for &source in sources {
        'inner: for curr in lower_idx..=ranges.len() {
            if curr < ranges.len() && ranges[curr].source_range_start < source {
                continue 'inner;
            } else {
                lower_idx = curr;
                if curr > 0 {
                    let possible_range = ranges[curr - 1];
                    let diff = source - possible_range.source_range_start;
                    if diff <= possible_range.range_length {
                        destinations.push(possible_range.destination_range_start + diff);
                    }
                }
                continue 'outer;
            }
        }
        destinations.push(source);
    }
    destinations
}

fn parse(mut lines: Lines<'_>) -> Almanac {
    // seeds
    let seeds = parse_seeds(lines.next().unwrap());
    let _ = lines.next();
    // seeds2soil
    assert_eq!("seed-to-soil map:", lines.next().unwrap());
    let seeds_to_soil = parse_block(&mut lines);

    // soil2fertilizer
    assert_eq!("soil-to-fertilizer map:", lines.next().unwrap());
    let soil_to_fertilizer = parse_block(&mut lines);

    // fertilizer2water
    assert_eq!("fertilizer-to-water map:", lines.next().unwrap());
    let fertilizer_to_water = parse_block(&mut lines);

    // water2light
    assert_eq!("water-to-light map:", lines.next().unwrap());
    let water_to_light = parse_block(&mut lines);

    // light2temperature
    assert_eq!("light-to-temperature map:", lines.next().unwrap());
    let light_to_temperature = parse_block(&mut lines);

    // temperature2humidity
    assert_eq!("temperature-to-humidity map:", lines.next().unwrap());
    let temperature_to_humidity = parse_block(&mut lines);

    // humidity2location
    assert_eq!("humidity-to-location map:", lines.next().unwrap());
    let humidity_to_location = parse_block(&mut lines);

    Almanac {
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn parse_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u64>().unwrap())
        .collect()
}

fn parse_block(lines: &mut Lines<'_>) -> Vec<Range> {
    lines
        .take_while(|line| !line.trim().is_empty())
        .map(|line| parse_range(line))
        .collect()
}

fn parse_range(range: &str) -> Range {
    let split: Vec<u64> = range
        .split_whitespace()
        .map(|val| val.parse::<u64>().unwrap())
        .collect();
    Range {
        destination_range_start: split[0],
        source_range_start: split[1],
        range_length: split[2],
    }
}

fn sort_by_source_id(rangy_vec: &mut Vec<Range>) {
    rangy_vec.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));
}

fn sort_by_destination_id(rangy_vec: &mut Vec<Range>) {
    rangy_vec.sort_by(|a, b| a.destination_range_start.cmp(&b.destination_range_start));
}

#[cfg(test)]
pub mod test05 {
    use std::collections::VecDeque;

    use crate::day05::{extend_ranges, parse, solve_p2, Almanac, Range};

    #[test]
    pub fn test_p2() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
            .lines();
        let mut almanac = parse(input);
        let orig_seeds = almanac.seeds.clone();
        almanac.sort_by_source();
        // let res_p2 = solve_p2(&almanac, orig_seeds);
        // assert_eq!(res_p2, 46);

        let res_test_val = solve_p2(&almanac, vec![82, 0]);
        assert_eq!(res_test_val, 46);
    }

    #[test]
    fn test_extend() {
        let mut sources = VecDeque::new();
        sources.push_front((5, 23));
        let mut ranges = Vec::new();
        let range1 = Range {
            source_range_start: 10,
            destination_range_start: 110,
            range_length: 5,
        };
        let range2 = Range {
            source_range_start: 16,
            destination_range_start: 216,
            range_length: 5,
        };
        ranges.push(range1);
        ranges.push(range2);
        let res = extend_ranges(sources, &ranges);
        println!("{:?}", res);
    }

    #[test]
    fn test_extend_2() {
        let mut sources = VecDeque::new();
        sources.push_front((9, 90));
        let mut ranges = Vec::new();
        let range1 = Range {
            source_range_start: 10,
            destination_range_start: 105,
            range_length: 10,
        };
        let range2 = Range {
            source_range_start: 50,
            destination_range_start: 100,
            range_length: 40,
        };
        ranges.push(range1);
        ranges.push(range2);
        let res = extend_ranges(sources, &ranges);
        println!("{:?}", res);
    }

    #[test]
    fn test_failing() {
        let mut almanac = provide_test_almanac();
        println!("ltt {:?}", &almanac.light_to_temperature);
        almanac.sort_by_source();
        let mut test_source = VecDeque::new();
        test_source.push_front((77, 77));

        println!("ltt {:?}", &almanac.light_to_temperature);
        println!("res {:?}", test_source);
        let mut res = extend_ranges(test_source, &almanac.light_to_temperature);

        println!("res {:?}", res);
        assert_eq!(res.pop_front(), Some((45, 45)))
    }

    fn provide_test_almanac() -> Almanac {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
            .lines();
        parse(input)
    }
}
