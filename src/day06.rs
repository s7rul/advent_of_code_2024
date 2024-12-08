use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next(pos: &(i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn rotate(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // find guard
    let mut guard_pos: (i32, i32) = (0, 0);

    for (l, line) in input.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == '^' {
                guard_pos = (l as i32, c as i32);
            }
        }
    }

    let max_l = input.len() as i32;
    let max_c = input[0].len() as i32;

    let mut guard_dir = Direction::Up;

    'out: loop {
        visited.insert(guard_pos);

        let mut next_pos = get_next(&guard_pos, guard_dir);

        if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
            break;
        }

        while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            guard_dir = rotate(guard_dir);
            next_pos = get_next(&guard_pos, guard_dir);
            if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
                break 'out;
            }
        }

        guard_pos = next_pos;
    }

    visited.len()
}

#[aoc(day6, part2, reduced_brute_force)]
pub fn solve_part2_1(input: &[Vec<char>]) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // find guard
    let mut guard_start: (i32, i32) = (0, 0);

    for (l, line) in input.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == '^' {
                guard_start = (l as i32, c as i32);
            }
        }
    }
    // find guard
    let mut guard_pos: (i32, i32) = guard_start;

    for (l, line) in input.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == '^' {
                guard_pos = (l as i32, c as i32);
            }
        }
    }

    let max_l = input.len() as i32;
    let max_c = input[0].len() as i32;

    let mut guard_dir = Direction::Up;

    'out: loop {
        visited.insert(guard_pos);

        let mut next_pos = get_next(&guard_pos, guard_dir);

        if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
            break;
        }

        while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            guard_dir = rotate(guard_dir);
            next_pos = get_next(&guard_pos, guard_dir);
            if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
                break 'out;
            }
        }

        guard_pos = next_pos;
    }

    let max_l = input.len() as i32;
    let max_c = input[0].len() as i32;

    let mut sum = 0;

    for (l, c) in visited {
        if l == guard_start.0 && c == guard_start.1 {
            continue;
        }
        let mut loop_detected: HashSet<(i32, i32, Direction)> = HashSet::new();

        // place obstacle
        let mut input = input.to_vec();
        input[l as usize][c as usize] = '#';

        let mut guard_pos = guard_start;
        let mut guard_dir = Direction::Up;

        let loop_detected = 'out: loop {
            if loop_detected.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                break true;
            }
            loop_detected.insert((guard_pos.0, guard_pos.1, guard_dir));

            let mut next_pos = get_next(&guard_pos, guard_dir);

            if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
                break false;
            }

            while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                guard_dir = rotate(guard_dir);
                next_pos = get_next(&guard_pos, guard_dir);
                if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
                    break 'out false;
                }
            }

            guard_pos = next_pos;
        };

        if loop_detected {
            sum += 1;
        }
    }

    sum
}

#[aoc(day6, part2, brute_force)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    // find guard
    let mut guard_start: (i32, i32) = (0, 0);

    for (l, line) in input.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if *char == '^' {
                guard_start = (l as i32, c as i32);
            }
        }
    }

    let max_l = input.len() as i32;
    let max_c = input[0].len() as i32;

    let mut sum = 0;

    for (l, line) in input.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if l as i32 == guard_start.0 && c as i32 == guard_start.1 {
                continue;
            }
            let mut loop_detected: HashSet<(i32, i32, Direction)> = HashSet::new();

            // place obstacle
            let mut input = input.to_vec();
            input[l][c] = '#';

            let mut guard_pos = guard_start;
            let mut guard_dir = Direction::Up;

            let loop_detected = 'out: loop {
                if loop_detected.contains(&(guard_pos.0, guard_pos.1, guard_dir)) {
                    break true;
                }
                loop_detected.insert((guard_pos.0, guard_pos.1, guard_dir));

                let mut next_pos = get_next(&guard_pos, guard_dir);

                if next_pos.0 >= max_l || next_pos.1 >= max_c || next_pos.0 < 0 || next_pos.1 < 0 {
                    break false;
                }

                while input[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                    guard_dir = rotate(guard_dir);
                    next_pos = get_next(&guard_pos, guard_dir);
                    if next_pos.0 >= max_l
                        || next_pos.1 >= max_c
                        || next_pos.0 < 0
                        || next_pos.1 < 0
                    {
                        break 'out false;
                    }
                }

                guard_pos = next_pos;
            };

            if loop_detected {
                sum += 1;
            }
        }
    }

    sum
}
