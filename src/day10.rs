#![allow(clippy::comparison_chain)]
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.chars().map(|c| c as u8 - b'0').collect()).collect()
}

fn calculate_trailhead_score(visited: &mut HashSet<(usize, usize)>, map: &[Vec<u8>], pos_x: usize, pos_y: usize) -> u64 {
    if visited.contains(&(pos_y, pos_x)) {
        return 0;
    }
    visited.insert((pos_y, pos_x));
    let current = map[pos_y][pos_x];
    if current == 9 {
        return 1;
    }
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut sum = 0;
    if pos_y > 0 && map[pos_y - 1][pos_x] == current + 1 {
        sum += calculate_trailhead_score(visited, map, pos_x, pos_y - 1);
    }
    if pos_y < max_y && map[pos_y + 1][pos_x] == current + 1 {
        sum += calculate_trailhead_score(visited, map, pos_x, pos_y + 1);
    }
    if pos_x > 0 && map[pos_y][pos_x - 1] == current + 1 {
        sum += calculate_trailhead_score(visited, map, pos_x - 1, pos_y);
    }
    if pos_x < max_x && map[pos_y][pos_x + 1] == current + 1 {
        sum += calculate_trailhead_score(visited, map, pos_x + 1, pos_y);
    }
    sum
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> u64 {
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                let mut visited = HashSet::new();
                sum += calculate_trailhead_score(&mut visited, input, x, y);
            }
        }
    }
    sum
}

fn calculate_trailhead_rating(map: &[Vec<u8>], pos_x: usize, pos_y: usize) -> u64 {
    let current = map[pos_y][pos_x];
    if current == 9 {
        return 1;
    }
    let max_x = map[0].len() - 1;
    let max_y = map.len() - 1;

    let mut sum = 0;
    if pos_y > 0 && map[pos_y - 1][pos_x] == current + 1 {
        sum += calculate_trailhead_rating(map, pos_x, pos_y - 1);
    }
    if pos_y < max_y && map[pos_y + 1][pos_x] == current + 1 {
        sum += calculate_trailhead_rating(map, pos_x, pos_y + 1);
    }
    if pos_x > 0 && map[pos_y][pos_x - 1] == current + 1 {
        sum += calculate_trailhead_rating(map, pos_x - 1, pos_y);
    }
    if pos_x < max_x && map[pos_y][pos_x + 1] == current + 1 {
        sum += calculate_trailhead_rating(map, pos_x + 1, pos_y);
    }
    sum
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> u64 {
    let mut sum = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 0 {
                sum += calculate_trailhead_rating(input, x, y);
            }
        }
    }
    sum
}

#[test]
fn test_1() {
    let input = generator("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732");
    let result = solve_part1(&input);
    assert_eq!(36, result);
}
