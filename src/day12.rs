#![allow(clippy::comparison_chain)]
use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|v| v.chars().collect()).collect()
}

#[derive(Debug)]
struct Plot {
    area: usize,
    perimiter: usize,
    up_perimiter_squares: Vec<(usize, usize)>,
    down_perimiter_squares: Vec<(usize, usize)>,
    left_perimiter_squares: Vec<(usize, usize)>,
    right_perimiter_squares: Vec<(usize, usize)>,
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

fn find_plots(input: &[Vec<char>]) -> Vec<Plot> {
    let mut plots: Vec<Plot> = vec![];

    let mut to_visit_outside: Vec<(usize, usize)> = vec![(0, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((next_x, next_y)) = to_visit_outside.pop() {
        let mut to_visit_inside: Vec<(usize, usize)> = vec![(next_x, next_y)];
        let mut perimiter = 0;
        let mut area = 0;

        let mut up_perimiter_squares = vec![];
        let mut down_perimiter_squares = vec![];
        let mut left_perimiter_squares = vec![];
        let mut right_perimiter_squares = vec![];

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
                up_perimiter_squares.push((next_x, next_y));
            } else if input[next_y - 1][next_x] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x, next_y - 1));
                up_perimiter_squares.push((next_x, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x, next_y - 1));
            }

            // down
            if next_y == input.len() - 1 {
                perimiter += 1;
                down_perimiter_squares.push((next_x, next_y));
            } else if input[next_y + 1][next_x] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x, next_y + 1));
                down_perimiter_squares.push((next_x, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x, next_y + 1));
            }

            // left
            if next_x == 0 { 
                perimiter += 1;
                left_perimiter_squares.push((next_x, next_y));
            } else if input[next_y][next_x - 1] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x - 1, next_y));
                left_perimiter_squares.push((next_x, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x - 1, next_y));
            }

            // right
            if next_x == input[0].len() - 1 { 
                perimiter += 1;
                right_perimiter_squares.push((next_x, next_y));
            } else if input[next_y][next_x + 1] != plant {
                perimiter += 1;
                add_if_not_in(&visited, &mut to_visit_outside, (next_x + 1, next_y));
                right_perimiter_squares.push((next_x, next_y));
            } else {
                add_if_not_in(&visited, &mut to_visit_inside, (next_x + 1, next_y));
            }
        }
        plots.push(Plot {perimiter, area, left_perimiter_squares, right_perimiter_squares, up_perimiter_squares, down_perimiter_squares});
    }
    plots
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let plots = find_plots(input);
    plots.iter().map(|p| p.get_price()).sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    let plots = find_plots(input);
    let mut sum = 0;
    for mut plot in plots {
        let mut sides = 0;
        plot.up_perimiter_squares.sort_by(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.0.cmp(&b.0),
                Ordering::Greater => Ordering::Greater,
            }
        });

        let mut last_square = plot.up_perimiter_squares[0];
        sides += 1;
        for square in &plot.up_perimiter_squares[1..] {
            if square.0 != last_square.0 + 1 || square.1 != last_square.1 {
                sides += 1;
            }
            last_square = *square;
        }

        plot.down_perimiter_squares.sort_by(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.0.cmp(&b.0),
                Ordering::Greater => Ordering::Greater,
            }
        });

        let mut last_square = plot.down_perimiter_squares[0];
        sides += 1;
        for square in &plot.down_perimiter_squares[1..] {
            if square.0 != last_square.0 + 1 || square.1 != last_square.1 {
                sides += 1;
            }
            last_square = *square;
        }

        plot.left_perimiter_squares.sort_by(|a, b| {
            match a.0.cmp(&b.0) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Greater => Ordering::Greater,
            }
        });

        let mut last_square = plot.left_perimiter_squares[0];
        sides += 1;
        for square in &plot.left_perimiter_squares[1..] {
            if square.1 != last_square.1 + 1 || square.0 != last_square.0 {
                sides += 1;
            }
            last_square = *square;
        }

        plot.right_perimiter_squares.sort_by(|a, b| {
            match a.0.cmp(&b.0) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Greater => Ordering::Greater,
            }
        });

        let mut last_square = plot.right_perimiter_squares[0];
        sides += 1;
        for square in &plot.right_perimiter_squares[1..] {
            if square.1 != last_square.1 + 1 || square.0 != last_square.0 {
                sides += 1;
            }
            last_square = *square;
        }

        sum += plot.area * sides;
    }
    sum
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

#[test]
fn test_2_1() {
    let input = generator("AAAA
BBCD
BBCC
EEEC");
    let result = solve_part2(&input);
    assert_eq!(80, result);
}
