use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let start = Instant::now();
    println!("Day 8");
    let input = read_input_to_vec("input/day8.txt");
    let (directions, map) = parse(&input);
    let result = part1(&directions, &map);
    println!("Part1: {result}");
    let (starting, directions, map) = parse2(&input);
    let result = part2(&starting, &directions, &map);
    println!("part 2: {result}");
    println!("time: {:?}", start.elapsed());
}

fn check_end2(keys: &Vec<String>) -> bool {
    for key in keys {
        let end = key.chars().collect::<Vec<char>>()[2];
        if end != 'Z' {
            return false;
        }
    }
    true
}

fn cycle_detector(
    starting: &String,
    directions: &Vec<char>,
    map: &HashMap<String, Node>,
) -> PathCycle {
    let mut visited: HashMap<(String, usize), u64> = HashMap::new();
    let mut next = starting.to_owned();
    let mut i = 0;
    let mut cycle_start = 0;
    let mut cycle_lenght = 0;
    let mut ends = vec![];
    let mut counter = 0;
    loop {
        let node = map.get(&next).unwrap();
        if visited.contains_key(&(next.clone(), i)) {
            cycle_start = *visited.get(&(next.clone(), i)).unwrap();
            cycle_lenght = counter - cycle_start;
            break;
        } else {
            visited.insert((next.clone(), i), counter);
            let end = next.chars().collect::<Vec<char>>()[2];
            if end == 'Z' {
                ends.push(counter);
            }
        }
        next = match directions[i] {
            'L' => node.0.clone(),
            'R' => node.1.clone(),
            _ => panic!(),
        };
        i = if i + 1 < directions.len() { i + 1 } else { 0 };
        counter += 1;
    }
    PathCycle {
        start: cycle_start,
        length: cycle_lenght,
        ends: ends
            .iter()
            .filter(|v| **v >= cycle_start)
            .map(|v| *v - cycle_start)
            .collect(),
    }
}

#[test]
fn test_check_end2() {
    let to_check = vec!["AAA".to_owned(), "ABZ".to_owned(), "BZE".to_owned()];
    assert_eq!(check_end2(&to_check), false);
    let to_check = vec!["AZA".to_owned(), "AZR".to_owned(), "BZE".to_owned()];
    assert_eq!(check_end2(&to_check), false);
    let to_check = vec!["AAZ".to_owned(), "ARZ".to_owned(), "BEZ".to_owned()];
    assert_eq!(check_end2(&to_check), true);
}

#[derive(Debug)]
struct PathCycle {
    start: u64,
    length: u64,
    ends: Vec<u64>,
}

fn check_path(end: u64, other: &PathCycle) -> bool {
    let cycles = (end - other.start) / other.length;
    let other_start = other.start + other.length * cycles;

    for other_end in &other.ends {
        let other_end = other_end + other_start;
        if other_end == end {
            return true;
        }
    }
    false
}

fn check_paths(end: u64, others: &[PathCycle]) -> bool {
    for other in others {
        if !check_path(end, other) {
            return false;
        }
    }
    true
}

fn part2(starting: &Vec<String>, directions: &Vec<char>, map: &HashMap<String, Node>) -> u64 {
    let paths: Vec<PathCycle> = starting
        .iter()
        .map(|start| cycle_detector(start, directions, map))
        .collect();

    let mut i: u64 = 0;
    loop {
        let start = paths[0].start + i * paths[0].length;
        for end in &paths[0].ends {
            let to_check = *end + start;

            if check_paths(to_check, &paths[1..]) {
                return to_check;
            }
        }
        i += 1;
    }
}

fn part2_bruteforce(
    mut starting: Vec<String>,
    directions: &Vec<char>,
    map: &HashMap<String, Node>,
) -> u64 {
    let mut counter = 0;
    let mut i = 0;
    while !check_end2(&starting) {
        for n in 0..starting.len() {
            let next = &starting[n];
            let node = map.get(next).unwrap();
            let next = match directions[i] {
                'L' => node.0.clone(),
                'R' => node.1.clone(),
                _ => panic!(),
            };
            starting[n] = next;
        }
        i = if i + 1 < directions.len() { i + 1 } else { 0 };
        counter += 1;
    }
    counter
}

fn part1(directions: &Vec<char>, map: &HashMap<String, Node>) -> u64 {
    let mut counter = 0;
    let mut next = "AAA".to_owned();
    let mut i = 0;
    while next != "ZZZ" {
        let node = map.get(&next).unwrap();
        next = match directions[i] {
            'L' => node.0.clone(),
            'R' => node.1.clone(),
            _ => panic!(),
        };
        i = if i + 1 < directions.len() { i + 1 } else { 0 };
        counter += 1;
    }
    counter
}

fn parse2(input: &Vec<String>) -> (Vec<String>, Directions, HashMap<String, Node>) {
    let directions = input[0].chars().collect();
    let mut starting = vec![];

    let mut map = HashMap::new();
    for line in &input[2..] {
        let split: Vec<&str> = line.split_whitespace().collect();
        let key = split[0].to_owned();
        let left = split[2][1..4].to_owned();
        let right = split[3][..3].to_owned();
        let end = key.chars().collect::<Vec<char>>()[2];
        if end == 'A' {
            starting.push(key.clone());
        }
        map.insert(key, (left, right));
    }

    (starting, directions, map)
}

type Node = (String, String);
type Directions = Vec<char>;

fn parse(input: &Vec<String>) -> (Directions, HashMap<String, Node>) {
    let directions = input[0].chars().collect();

    let mut map = HashMap::new();
    for line in &input[2..] {
        let split: Vec<&str> = line.split_whitespace().collect();
        let key = split[0].to_owned();
        let left = split[2][1..4].to_owned();
        let right = split[3][..3].to_owned();
        map.insert(key, (left, right));
    }

    (directions, map)
}
