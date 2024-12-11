#![allow(clippy::comparison_chain)]
use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|v| v.parse().unwrap()).collect()
}

fn count_digits(v: u64) -> u64 {
    let mut n: u64 = 0;
    while v / 10_u64.pow(n as u32) > 0 {
        n += 1;
    }
    n
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[u64]) -> usize {
    let mut stones = input.to_vec();
    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
                i += 1;
            } else {
                let digits = count_digits(stone);
                if digits % 2 == 0 {
                    let left = stone / 10_u64.pow(digits as u32 / 2);
                    let right = stone % 10_u64.pow(digits as u32 / 2);
                    stones[i] = right;
                    stones.insert(i, left);
                    i += 2;
                } else {
                    stones[i] = stone * 2024;
                    i += 1;
                }
            }
        }
    }
    stones.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[u64]) -> usize {
    let mut sum = 0;
    for (n, s) in input.iter().enumerate() {
        let mut stones = vec![*s];
        for _ in 0..75 {
            let mut i = 0;
            while i < stones.len() {
                let stone = stones[i];
                if stone == 0 {
                    stones[i] = 1;
                    i += 1;
                } else {
                    let digits = count_digits(stone);
                    if digits % 2 == 0 {
                        let left = stone / 10_u64.pow(digits as u32 / 2);
                        let right = stone % 10_u64.pow(digits as u32 / 2);
                        stones[i] = right;
                        stones.insert(i, left);
                        i += 2;
                    } else {
                        stones[i] = stone * 2024;
                        i += 1;
                    }
                }
            }
        }
        sum += stones.len();
        println!("n: {n} sum: {sum}");
    }
    sum
}
