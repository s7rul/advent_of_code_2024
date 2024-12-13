#![allow(clippy::comparison_chain)]
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Move {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Machine {
    button_a: Move,
    button_b: Move,
    price: Point,
}

impl Machine {
    fn from_str(input: &str) -> Machine {
        let mut lines = input.lines();
        let move_a_vec: Vec<i64>= lines.next().unwrap().split_once(':').unwrap().1.split(',').map(|v| v.trim()[1..].parse().unwrap()).collect();
        let move_a = Move {x: move_a_vec[0], y: move_a_vec[1]};
        let move_b_vec: Vec<i64>= lines.next().unwrap().split_once(':').unwrap().1.split(',').map(|v| v.trim()[1..].parse().unwrap()).collect();
        let move_b = Move {x: move_b_vec[0], y: move_b_vec[1]};
        let price_vec: Vec<i64>= lines.next().unwrap().split_once(':').unwrap().1.split(',').map(|v| v.trim()[2..].parse().unwrap()).collect();
        let price = Point {x: price_vec[0], y: price_vec[1]};
        Machine {
            button_a: move_a, button_b: move_b, price
        }
    }

    fn lowest_price_constant(&self) -> u64 {
        let upper = self.price.y * self.button_a.x - self.price.x * self.button_a.y;
        let lower = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;

        if upper % lower != 0 {
            return 0;
        }

        let bp = upper / lower;

        if bp < 0 {
            return 0;
        }

        let upper = self.price.x - bp * self.button_b.x;
        let lower = self.button_a.x;

        if upper % lower != 0 {
            return 0;
        }

        let ap = upper / lower;
        
        if ap < 0 {
            return 0;
        }

        (ap * 3 + bp) as u64
    }
}

#[aoc_generator(day13, part1)]
pub fn generator(input: &str) -> Vec<Machine> {
    input.split("\n\n").map(Machine::from_str).collect()
}

#[aoc_generator(day13, part2)]
pub fn generator2(input: &str) -> Vec<Machine> {
    input.split("\n\n").map(|v| {
        let mut m = Machine::from_str(v);
        m.price.x += 10000000000000;
        m.price.y += 10000000000000;
        m
    }).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Machine]) -> u64 {
    input.iter().map(|m| m.lowest_price_constant()).sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Machine]) -> u64 {
    input.iter().map(|m| m.lowest_price_constant()).sum()
}
