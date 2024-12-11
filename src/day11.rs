#![allow(clippy::comparison_chain)]
use std::{collections::HashMap, usize};

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

fn recursive(input: u64, blink: u64, memory: &mut HashMap<(u64, u64), usize>) -> usize {
    if blink == 75 {
        1
    } else if let Some(v) = memory.get(&(input, blink)) {
        *v
    } else {
        let result = if input == 0 {
            recursive(1, blink + 1, memory)
        } else {
            let digits = count_digits(input);
            if digits % 2 == 0 {
                recursive(input / 10_u64.pow(digits as u32 / 2), blink + 1, memory) +
                recursive(input % 10_u64.pow(digits as u32 / 2), blink + 1, memory)
            } else {
                recursive(input * 2024, blink + 1, memory)
            }
        };
        memory.insert((input, blink), result);
        result
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[u64]) -> usize {
    let mut sum = 0;
    let mut memory = HashMap::new();
    for (n, s) in input.iter().enumerate() {
        sum += recursive(*s, 0, &mut memory);
    }
    sum
}
