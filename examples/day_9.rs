use std::time::Instant;

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let start = Instant::now();
    println!("Day 9");
    let input = read_input_to_vec("input/day9.txt");
    let data = parse(&input);
    let result = part1(&data);
    println!("Part1: {result}");
    let result = part2(&data);
    println!("Part2: {result}");
    println!("time: {:?}", start.elapsed());
}

fn check_zeroes(input: &Vec<i64>) -> bool {
    for i in input {
        if *i != 0 {
            return false;
        }
    }
    true
}

fn get_next(input: &Vec<i64>) -> i64 {
    if check_zeroes(input) {
        0
    } else {
        let mut new_series = vec![];
        for i in 1..input.len() {
            new_series.push(input[i] - input[i - 1]);
        }
        input[input.len() - 1] + get_next(&new_series)
    }
}

fn get_previous(input: &Vec<i64>) -> i64 {
    if check_zeroes(input) {
        0
    } else {
        let mut new_series = vec![];
        for i in 1..input.len() {
            new_series.push(input[i] - input[i - 1]);
        }
        input[0] - get_previous(&new_series)
    }
}

fn part1(data: &Vec<Vec<i64>>) -> i64 {
    let mut results = vec![];

    for series in data {
        results.push(get_next(series));
    }

    results.iter().sum()
}

fn part2(data: &Vec<Vec<i64>>) -> i64 {
    let mut results = vec![];

    for series in data {
        results.push(get_previous(series));
    }

    results.iter().sum()
}

fn parse(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut ret = vec![];
    for line in input {
        ret.push(
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        )
    }
    ret
}
