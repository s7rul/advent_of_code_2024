use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec = vec![];
    let mut right_vec = vec![];
    for l in input.lines() {
        let mut split = l.split_whitespace();
        let left: i32 = split.next().unwrap().parse().unwrap();
        let right: i32 = split.next().unwrap().parse().unwrap();
        left_vec.push(left);
        right_vec.push(right);
    }
    (left_vec, right_vec)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();
    left.sort();
    right.sort();

    let mut dist = 0;

    for i in 0..(left.len()) {
        dist += (left[i] - right[i]).abs();
    }

    dist
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let left = input.0.clone();
    let right = input.1.clone();

    let mut num_of_num: HashMap<i32, i32> = HashMap::new();
    for n in right {
        match num_of_num.get(&n) {
            Some(v) => {
                num_of_num.insert(n, v + 1);
            }
            None => {
                num_of_num.insert(n, 1);
            }
        }
    }

    let mut sum = 0;
    for n in left {
        sum += n * match num_of_num.get(&n) {
            Some(v) => *v,
            None => 0,
        };
    }
    sum
}
