use core::panic;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Antenna {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Antinode {
    x: i32,
    y: i32,
}

pub struct Map {
    max_x: u32,
    max_y: u32,
    antennas: HashMap<char, Vec<Antenna>>,
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Map {
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, l) in input.lines().enumerate() {
        max_x = l.len() as u32;
        max_y = max_y.max(y as u32);

        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| {
                        v.push(Antenna {
                            x: x as i32,
                            y: y as i32,
                        })
                    })
                    .or_insert(vec![Antenna {
                        x: x as i32,
                        y: y as i32,
                    }]);
            }
        }
    }

    Map {
        max_x,
        max_y: max_y + 1,
        antennas,
    }
}

fn calculate_antinodes(first: &Antenna, secound: &Antenna) -> [Antinode; 2] {
    let diff_x = (first.x - secound.x).abs();
    let diff_y = (first.y - secound.y).abs();
    if first.x <= secound.x && first.y <= secound.y {
        [Antinode {x: first.x - diff_x, y: first.y - diff_y}, Antinode {x: secound.x + diff_x, y: secound.y + diff_y}]
    } else if first.x >= secound.x && first.y <= secound.y {
        [Antinode {x: first.x + diff_x, y: first.y - diff_y}, Antinode {x: secound.x - diff_x, y: secound.y + diff_y}]
    } else if first.x <= secound.x && first.y >= secound.y {
        [Antinode {x: first.x - diff_x, y: first.y + diff_y}, Antinode {x: secound.x + diff_x, y: secound.y - diff_y}]
    } else {
        [Antinode {x: first.x + diff_x, y: first.y + diff_y}, Antinode {x: secound.x - diff_x, y: secound.y - diff_y}]
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for e in input.antennas.values() {
        for i in 0..e.len() {
            for j in (i+1)..e.len() {
                let a = calculate_antinodes(&e[i], &e[j]);
                for ant in a {
                    if ant.x >= 0 && ant.y >= 0 && ant.x < input.max_x as i32 && ant.y < input.max_y as i32 {
                        antinodes.insert((ant.x, ant.y));
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for e in input.antennas.values() {
        for i in 0..e.len() {
            for j in (i+1)..e.len() {
                let a = calculate_antinodes_with_harmonics(input.max_x as i32, input.max_y as i32, &e[i], &e[j]);
                for ant in a {
                    antinodes.insert((ant.x, ant.y));
                }
            }
        }
    }
    antinodes.len()
}

fn calculate_antinodes_with_harmonics(max_x: i32, max_y: i32, first: &Antenna, secound: &Antenna) -> Vec<Antinode> {
    let mut ret = vec![];
    let diff_x = (first.x - secound.x).abs();
    let diff_y = (first.y - secound.y).abs();
    if first.x <= secound.x && first.y <= secound.y {
        let mut i = 0;
        loop {
            let ant = Antinode {x: first.x - diff_x * i, y: first.y - diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
        i = 0;
        loop {
            let ant = Antinode {x: secound.x + diff_x * i, y: secound.y + diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
    } else if first.x >= secound.x && first.y <= secound.y {
        let mut i = 0;
        loop {
            let ant = Antinode {x: first.x + diff_x * i, y: first.y - diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
        i = 0;
        loop {
            let ant = Antinode {x: secound.x - diff_x * i, y: secound.y + diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
    } else if first.x <= secound.x && first.y >= secound.y {
        let mut i = 0;
        loop {
            let ant = Antinode {x: first.x - diff_x * i, y: first.y + diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
        i = 0;
        loop {
            let ant = Antinode {x: secound.x + diff_x * i, y: secound.y - diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
    } else {
        let mut i = 0;
        loop {
            let ant = Antinode {x: first.x + diff_x * i, y: first.y + diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
        i = 0;
        loop {
            let ant = Antinode {x: secound.x - diff_x * i, y: secound.y - diff_y * i};
            if !(ant.x >= 0 && ant.y >= 0 && ant.x < max_x && ant.y < max_y) {
                break;
            }
            ret.push(ant);
            i += 1;
        }
    }
    ret
}

#[test]
fn test_22() {
    let input = generator("T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........");
    println!("max x: {}, y: {}", input.max_x, input.max_y);
    let result = solve_part2(&input);
    assert_eq!(9, result);
}

#[test]
fn test_22_2() {
    let input = generator("T.........
...T......
..........
..........
..........
..........
..........
..........
..........
..........");
    println!("max x: {}, y: {}", input.max_x, input.max_y);
    let result = solve_part2(&input);
    assert_eq!(4, result);
}

#[test]
fn test_1() {
    let input = generator("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
    println!("max x: {}, y: {}", input.max_x, input.max_y);
    let result = solve_part1(&input);
    assert_eq!(14, result);
}

#[test]
fn test_1_2() {
    let input = generator("............
............
............
.......0....
....0.......
............
............
............
............
............
............
............");
    let result = solve_part1(&input);
    assert_eq!(2, result);
}

#[test]
fn test_2() {
    let input = generator("..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........");
    let result = solve_part1(&input);
    assert_eq!(2, result);
}

#[test]
fn test_3() {
    let input = generator("..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
);
    let result = solve_part1(&input);
    assert_eq!(4, result);
}

#[test]
fn test_4() {
    let input = generator("..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."
);
    let result = solve_part1(&input);
    assert_eq!(4, result);
}
