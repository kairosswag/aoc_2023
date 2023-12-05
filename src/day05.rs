use std::fs;
use std::str::Lines;

#[derive(Copy, Clone, Debug)]
struct Range {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

struct Almanac {
    seeds: Vec<u32>,
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

    almanac.sort_by_source();
    let res = solve_p1(&almanac);

    let res = (res, 5);
    println!("Day 05 Solution Part 1: {}", res.0);
    println!("Day 05 Solution Part 2: {}", res.1);
}

fn solve_p1(almanac: &Almanac) -> u32 {
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

    println!("location {:?}", location);
    location[0]
}

fn get_destinations(sources: &[u32], ranges: &[Range]) -> Vec<u32> {
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

fn parse_seeds(seeds: &str) -> Vec<u32> {
    seeds
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<u32>().unwrap())
        .collect()
}

fn parse_block(lines: &mut Lines<'_>) -> Vec<Range> {
    lines
        .take_while(|line| !line.trim().is_empty())
        .map(|line| parse_range(line))
        .collect()
}

fn parse_range(range: &str) -> Range {
    let split: Vec<u32> = range
        .split_whitespace()
        .map(|val| val.parse::<u32>().unwrap())
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
