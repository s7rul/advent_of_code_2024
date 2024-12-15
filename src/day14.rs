#![allow(clippy::comparison_chain)]
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn move_one_step<const DIM_X: i32, const DIM_Y: i32>(&mut self) {
        let new_x = self.position.x + self.velocity.x;
        let new_y = self.position.y + self.velocity.y;
        self.position.x = ((new_x % DIM_X) + DIM_X) % DIM_X;
        self.position.y = ((new_y % DIM_Y) + DIM_Y) % DIM_Y;
    }
}

fn move_robots_one_step<const DIM_X: i32, const DIM_Y: i32>(robots: &mut[Robot]) {
    for r in robots {
        r.move_one_step::<DIM_X, DIM_Y>()
    }
}

fn calculate_safety_factor<const DIM_X: i32, const DIM_Y: i32>(robots: &mut[Robot]) -> u32 {
    let mut end_position: HashMap<Position, u32> = HashMap::new();

    for robot in robots {
        for _ in 0..100 {
            robot.move_one_step::<DIM_X, DIM_Y>();
        }
        end_position.entry(robot.position).and_modify(|v| *v += 1).or_insert(1);
    }
    
    let mut quad_lu = 0;
    let mut quad_ld = 0;
    let mut quad_ru= 0;
    let mut quad_rd = 0;

    for (pos, n) in end_position {
        if pos.x < DIM_X / 2 {
            if pos.y < DIM_Y / 2 {
                quad_lu += n;
            } else if pos.y > (DIM_Y / 2) {
                quad_ld += n;
            }
        } else if pos.x > (DIM_X / 2) {
            if pos.y < DIM_Y / 2 {
                quad_ru += n;
            } else if pos.y > (DIM_Y / 2) {
                quad_rd += n;
            }
        }
    }
    println!("lu: {quad_lu}, ld: {quad_ld} ru: {quad_ru} rd: {quad_rd}");
    quad_lu * quad_ld * quad_ru * quad_rd
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Robot> {
    input.lines().map(|l| {
        let mut s = l.split_whitespace();
        let pos_str = s.next().unwrap().split_once('=').unwrap().1.split_once(',').unwrap();
        let position = Position {x: pos_str.0.parse().unwrap(), y: pos_str.1.parse().unwrap()};
        let pos_str = s.next().unwrap().split_once('=').unwrap().1.split_once(',').unwrap();
        let velocity = Velocity {x: pos_str.0.parse().unwrap(), y: pos_str.1.parse().unwrap()};
        Robot { position, velocity }
    }).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Robot]) -> u32 {
    let mut robots = input.to_vec();;
    calculate_safety_factor::<101, 103>(&mut robots)
}

fn print_map(robots: &[Robot]) {
    let mut map = [['.'; 101];103];

    for robot in robots {
        map[robot.position.y as usize][robot.position.x as usize] = '#';
    }

    for l in map {
        for c in l {
            print!("{c}");
        }
        println!();
    }
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Robot]) -> u32 {
    let mut robots = input.to_vec();;
    for n in 1..100001 {
        move_robots_one_step::<101, 103>(&mut robots);
        println!("N: {n}");
        print_map(&robots);
    }
    0
}

#[test]
fn test_1() {
    let mut input = generator("p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3");
    let result = calculate_safety_factor::<11, 7>(&mut input);
    assert_eq!(12, result);
}
