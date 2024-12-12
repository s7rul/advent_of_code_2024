#![allow(clippy::comparison_chain)]
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|v| v.chars().collect()).collect()
}

#[derive(Debug)]
struct Plot {
    area: usize,
    perimiter: usize,
}

impl Plot {
    fn get_price(&self) -> usize {
        self.area * self.perimiter
    }
}

fn add_if_not_in(visited: &HashSet<(usize, usize)>, list: &mut Vec<(usize, usize)>, element: (usize, usize)) {
    if !visited.contains(&element) && !list.iter().any(|(x, y)| *x == element.0 && *y == element.1) {
        list.push(element);
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let mut plots: Vec<Plot> = vec![];

    let mut to_visit_outside: Vec<(usize, usize)> = vec![(0, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((next_x, next_y)) = to_visit_outside.pop() {
        let mut to_visit_inside: Vec<(usize, usize)> = vec![(next_x, next_y)];
        let mut perimiter = 0;
        let mut area = 0;

        while let Some((next_x, next_y)) = to_visit_inside.pop() {
            visited.insert((next_x, next_y));

            area += 1;
            // remove this from outside list
            if let Some(i) = to_visit_outside.iter().position(|(x, y)| *x == next_x && *y == next_y) {
                to_visit_outside.remove(i);
            }

            // look around
            let plant = input[next_y][next_x];

            // up
            if next_y == 0 {
                perimiter += 1;
            } else if input[next_y - 1][next_x] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x, next_y - 1));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x, next_y - 1));
            }

            // down
            if next_y == input.len() - 1 {
                perimiter += 1;
            } else if input[next_y + 1][next_x] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x, next_y + 1));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x, next_y + 1));
            }

            // left
            if next_x == 0 { 
                perimiter += 1;
            } else if input[next_y][next_x - 1] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x - 1, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x - 1, next_y));
            }

            // right
            if next_x == input[0].len() - 1 { 
                perimiter += 1;
            } else if input[next_y][next_x + 1] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x + 1, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x + 1, next_y));
            }
        }
        plots.push(Plot {perimiter, area});
    }

    plots.iter().map(|p| p.get_price()).sum()
}

#[test]
fn test_1_1() {
    let input = generator("AAAA
BBCD
BBCC
EEEC");
    let result = solve_part1(&input);
    assert_eq!(140, result);
}
