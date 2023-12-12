use std::{collections::HashMap, time::Instant};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let start = Instant::now();
    println!("day 5");
    let input = read_input_to_vec("input/day5.txt");
    let (seeds, maps) = parse(&input);

    let mut current = seeds;

    for map in maps {
        for i in 0..current.len() {
            current[i] = map.convert(current[i])
        }
    }
    let min = current.iter().min().unwrap();
    println!("Part 1: {min}");

    let (seeds, maps) = parse2(&input);

    let mut to_process = seeds;
    for map in maps {
        let mut done_processing = vec![];

        // process
        while let Some(range) = to_process.pop() {
            let (prossesed, not_processed) = map.modifier(range);
            done_processing.push(prossesed);
            for range in not_processed {
                to_process.push(range);
            }
        }

        to_process = done_processing.clone();
        done_processing.clear();
    }

    let result = to_process
        .iter()
        .map(|range| range.get_min())
        .min()
        .unwrap();
    println!("Part 2: {result}");
    println!("Time: {:?}", start.elapsed());
}

fn parse(input: &Vec<String>) -> (Vec<i64>, Vec<Map>) {
    let seeds: Vec<i64> = input[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut ret = vec![];
    let mut current_map = vec![];
    for line in &input[1..] {
        if line == "" {
            if current_map.is_empty() {
                continue;
            } else {
                ret.push(Map {
                    mappings: current_map.clone(),
                });
                current_map.clear();
            }
        } else if line.chars().nth(0).unwrap().is_alphabetic() {
            continue;
        } else {
            let numbers: Vec<i64> = line
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            let dest_range_start = numbers[0];
            let source_range_start = numbers[1];
            let len = numbers[2];

            let mapping = Mapping {
                start: source_range_start,
                end: source_range_start + len,
                modifier: dest_range_start - source_range_start,
            };
            current_map.push(mapping);
        }
    }

    if !current_map.is_empty() {
        ret.push(Map {
            mappings: current_map,
        })
    }
    (seeds, ret)
}

#[derive(Debug, Clone)]
struct SeedRange {
    start: i64,
    end: i64,
}

impl SeedRange {
    fn get_min(&self) -> i64 {
        self.start
    }
}

fn parse2(input: &Vec<String>) -> (Vec<SeedRange>, Vec<Map>) {
    let seeds_numbers: Vec<i64> = input[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut seeds = vec![];
    for i in (0..(seeds_numbers.len())).step_by(2) {
        seeds.push(SeedRange {
            start: seeds_numbers[i],
            end: seeds_numbers[i] + seeds_numbers[i + 1],
        });
    }

    let mut ret = vec![];
    let mut current_map = vec![];
    for line in &input[1..] {
        if line == "" {
            if current_map.is_empty() {
                continue;
            } else {
                ret.push(Map {
                    mappings: current_map.clone(),
                });
                current_map.clear();
            }
        } else if line.chars().nth(0).unwrap().is_alphabetic() {
            continue;
        } else {
            let numbers: Vec<i64> = line
                .trim()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();

            let dest_range_start = numbers[0];
            let source_range_start = numbers[1];
            let len = numbers[2];

            let mapping = Mapping {
                start: source_range_start,
                end: source_range_start + len,
                modifier: dest_range_start - source_range_start,
            };
            current_map.push(mapping);
        }
    }

    if !current_map.is_empty() {
        ret.push(Map {
            mappings: current_map,
        })
    }
    (seeds, ret)
}

#[derive(Debug, Clone)]
struct Mapping {
    start: i64,
    end: i64,
    modifier: i64,
}

impl Mapping {
    fn contains(&self, input: i64) -> bool {
        input >= self.start && input < self.end
    }

    fn get_modifier(&self) -> i64 {
        self.modifier
    }

    fn intersects(&self, input: &SeedRange) -> bool {
        !(input.start >= self.end || input.end <= self.start)
    }

    fn get_new_ranges(&self, input: SeedRange) -> (Option<SeedRange>, Vec<SeedRange>) {
        if self.contains(input.start) && self.contains(input.end - 1) {
            (
                Some(SeedRange {
                    start: input.start + self.modifier,
                    end: input.end + self.modifier,
                }),
                vec![],
            )
        } else if self.contains(input.start) {
            let new_inside = SeedRange {
                start: input.start + self.modifier,
                end: self.end + self.modifier,
            };
            let new_outside = SeedRange {
                start: self.end,
                end: input.end,
            };
            (Some(new_inside), vec![new_outside])
        } else if self.contains(input.end - 1) {
            let new_inside = SeedRange {
                start: self.start + self.modifier,
                end: input.end + self.modifier,
            };
            let new_outside = SeedRange {
                start: input.start,
                end: self.start,
            };
            (Some(new_inside), vec![new_outside])
        } else if input.start < self.start && input.end > self.end {
            let new_inside = SeedRange {
                start: self.start + self.modifier,
                end: self.end + self.modifier,
            };
            let new_outside_right = SeedRange {
                start: self.end,
                end: input.end,
            };
            let new_outside_left = SeedRange {
                start: input.start,
                end: self.start,
            };
            (Some(new_inside), vec![new_outside_right, new_outside_left])
        } else {
            (None, vec![input])
        }
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn convert(&self, input: i64) -> i64 {
        for mapping in &self.mappings {
            if mapping.contains(input) {
                return input + mapping.modifier;
            }
        }
        input
    }

    fn modifier(&self, input: SeedRange) -> (SeedRange, Vec<SeedRange>) {
        for mapping in &self.mappings {
            if mapping.intersects(&input) {
                let ret = mapping.get_new_ranges(input);
                return (ret.0.unwrap(), ret.1);
            }
        }
        (input, vec![])
    }
}
