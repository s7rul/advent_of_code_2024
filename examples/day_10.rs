use std::{usize, collections::HashSet};

use advent_of_code_2022::read_input_to_vec;

#[test]
fn test_ex3_part2() {
    let input = read_input_to_vec("input/day10_test3.txt");
    let map = parse(&input);
    let result = find_loop(&map);
    let start = find_start(&(map.map[0].len(), map.map.len()), &map, &result.1, &result.2);
    let result = find_contained(&(map.map[0].len(), map.map.len()), &result.1, start);
    assert_eq!(result, 4);
}

#[test]
fn test_ex4_part2() {
    let input = read_input_to_vec("input/day10_test4.txt");
    let map = parse(&input);
    let result = find_loop(&map);
    let start = find_start(&(map.map[0].len(), map.map.len()), &map, &result.1, &result.2);
    let result = find_contained(&(map.map[0].len(), map.map.len()), &result.1, start);
    assert_eq!(result, 8);
}

#[test]
fn test_ex5_part2() {
    let input = read_input_to_vec("input/day10_test5.txt");
    let map = parse(&input);
    let result = find_loop(&map);
    let start = find_start(&(map.map[0].len(), map.map.len()), &map, &result.1, &result.2);
    let result = find_contained(&(map.map[0].len(), map.map.len()), &result.1, start);
    assert_eq!(result, 10);
}

#[test]
fn test_ex6_part2() {
    let input = read_input_to_vec("input/day10_test6.txt");
    let map = parse(&input);
    let result = find_loop(&map);
    let start = find_start(&(map.map[0].len(), map.map.len()), &map, &result.1, &result.2);
    let result = find_contained(&(map.map[0].len(), map.map.len()), &result.1, start);
    assert_eq!(result, 4);
}

fn main() {
    println!("Day 10");
    let input = read_input_to_vec("input/day10.txt");
    let map = parse(&input);
    let (length, tiles, dir) = find_loop(&map);
    let result = length / 2;
    println!("part1: {result}");
    let start = find_start(&(map.map[0].len(), map.map.len()), &map, &tiles, &dir);
    let result = find_contained(&(map.map[0].len(), map.map.len()), &tiles, start);
    println!("part2: {result}");
}

fn add_to_start(start: &mut Vec<(usize, usize)>, map: &Map, to_add: (usize, usize), in_loop: &HashSet<(usize, usize)>) {
    if to_add.0 >= map.map[0].len() || to_add.1 >= map.map.len() {
        return;
    }
    if !start.contains(&to_add) && !in_loop.contains(&to_add) {
        start.push(to_add);
    }
}

fn find_start(dim: &(usize, usize), map: &Map, in_loop: &HashSet<(usize, usize)>, dir: &Dir) -> Vec<(usize, usize)> {
    let mut start_right = vec![];
    let mut start_left = vec![];
    let (mut x, mut y) = map.start;
    let mut last_x = x;
    let mut last_y = y;

    let mut looking: Option<Looking> = None;

    loop {
        if x == last_x && y == last_y {
            (x, y) = match dir {
                Dir::S => (x, y+1),
                Dir::E => (x + 1, y),
                Dir::W => (x - 1, y),
                Dir::N => (x, y-1),
            };
        } else {
            let n = (x, y - 1);
            let s = (x, y + 1);
            let e = (x + 1, y);
            let w = (x - 1, y);
            let new_xy = match &map.map[y][x] {
                Part::Animal => break,
                Part::NS => {
                    if (last_x, last_y) == n {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x + 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x - 1, y), in_loop);
                        s
                    } else if (last_x, last_y) == s {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x - 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x + 1, y), in_loop);
                        n
                    } else {
                        panic!()
                    }
                },
                Part::EW => {
                    if (last_x, last_y) == e {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x, y + 1), in_loop);
                        add_to_start(&mut start_right, map, (x, y - 1), in_loop);
                        w
                    } else if (last_x, last_y) == w {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x, y - 1), in_loop);
                        add_to_start(&mut start_right, map, (x, y + 1), in_loop);
                        e
                    } else {
                        panic!()
                    }
                },
                Part::NE => {
                    if (last_x, last_y) == n {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_right, map, (x - 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x, y + 1), in_loop);
                        add_to_start(&mut start_right, map, (x - 1, y + 1), in_loop);
                        e
                    } else if (last_x, last_y) == e {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x - 1, y), in_loop);
                        add_to_start(&mut start_left, map, (x, y + 1), in_loop);
                        add_to_start(&mut start_left, map, (x - 1, y + 1), in_loop);
                        n
                    } else {
                        panic!()
                    }
                },
                Part::NW => {
                    if (last_x, last_y) == n {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x + 1, y), in_loop);
                        add_to_start(&mut start_left, map, (x, y + 1), in_loop);
                        add_to_start(&mut start_left, map, (x + 1, y + 1), in_loop);
                        w
                    } else if (last_x, last_y) == w {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::S) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_right, map, (x + 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x, y + 1), in_loop);
                        add_to_start(&mut start_right, map, (x + 1, y + 1), in_loop);
                        n
                    } else {
                        panic!()
                    }
                },
                Part::SW => {
                    if (last_x, last_y) == w {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x, y - 1), in_loop);
                        add_to_start(&mut start_left, map, (x + 1, y), in_loop);
                        add_to_start(&mut start_left, map, (x + 1, y - 1), in_loop);
                        s
                    } else if (last_x, last_y) == s {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::E) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_right, map, (x, y - 1), in_loop);
                        add_to_start(&mut start_right, map, (x + 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x + 1, y - 1), in_loop);
                        w
                    } else {
                        panic!()
                    }
                },
                Part::SE => {
                    if (last_x, last_y) == e {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Right);
                                } else if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Right);
                                }
                            },
                        }
                        add_to_start(&mut start_right, map, (x, y - 1), in_loop);
                        add_to_start(&mut start_right, map, (x - 1, y), in_loop);
                        add_to_start(&mut start_right, map, (x - 1, y - 1), in_loop);
                        s
                    } else if (last_x, last_y) == s {
                        match looking {
                            Some(_) => (),
                            None => {
                                if probe(map, in_loop, (x, y), Dir::W) {
                                    looking = Some(Looking::Left);
                                } else if probe(map, in_loop, (x, y), Dir::N) {
                                    looking = Some(Looking::Left);
                                }
                            },
                        }
                        add_to_start(&mut start_left, map, (x, y - 1), in_loop);
                        add_to_start(&mut start_left, map, (x - 1, y), in_loop);
                        add_to_start(&mut start_left, map, (x - 1, y - 1), in_loop);
                        e
                    } else {
                        panic!()
                    }
                },
                _ => {
                    panic!()
                }
            };
            (last_x, last_y) = (x, y);
            (x, y) = new_xy;
        }
    }
    match &looking {
        Some(l) => match l {
            Looking::Left => start_left,
            Looking::Right => start_right,
        },
        None => todo!(),
    }

}

fn find_contained(dim: &(usize, usize), in_loop: &HashSet<(usize, usize)>, start: Vec<(usize, usize)>) -> u64 {
    let mut explore = start;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(tile) = explore.pop() {
        visited.insert(tile.clone());
        let mut potentials = vec![];
        if tile.0 > 0 {
            potentials.push((tile.0 - 1, tile.1));
        }
        if tile.0 < dim.0 - 1 {
            potentials.push((tile.0 + 1, tile.1));
        }
        if tile.1 > 0 {
            potentials.push((tile.0, tile.1 - 1));
        }
        if tile.1 < dim.1 - 1 {
            potentials.push((tile.0, tile.1 + 1));
        }

        for t in potentials {
            if !in_loop.contains(&t) && !visited.contains(&t) && !explore.contains(&t) {
                explore.push(t);
            }
        }
    }

    (dim.0 * dim.1 - in_loop.len() - visited.len()) as u64
}

enum Looking {
    Left, Right
}

enum Dir {
    N, S, E, W
}

fn probe(map: &Map, in_loop: &HashSet<(usize, usize)>, start: (usize, usize), dir: Dir) -> bool {
    match dir {
        Dir::N => {
            for y in 0..start.1 {
                if in_loop.contains(&(start.0, y)) {
                    return false;
                }
            }
            true
        },
        Dir::S => {
            for y in start.1 + 1..map.map.len() {
                if in_loop.contains(&(start.0, y)) {
                    return false;
                }
            }
            true
        },
        Dir::E => {
            for x in start.0 + 1..map.map[0].len() {
                if in_loop.contains(&(x, start.1)) {
                    return false;
                }
            }
            true
        },
        Dir::W => {
            for x in 0..start.0 {
                if in_loop.contains(&(x, start.1)) {
                    return false;
                }
            }
            true
        },
    }
}

fn find_loop(map: &Map) -> (u64, HashSet<(usize, usize)>, Dir) {
    let mut in_loop = HashSet::new();
    let (mut x, mut y) = map.start;
    let mut last_x = x;
    let mut last_y = y;
    let mut i = 0;
    let mut dir = 0; //NSEW

    loop {
        if x == last_x && y == last_y {
            in_loop.clear();
            in_loop.insert((x, y));
            (x, y) = match dir {
                0 => (x, y+1),
                1 => (x + 1, y),
                2 => (x - 1, y),
                3 => (x, y-1),
                _ => panic!()
            };
        } else {
            in_loop.insert((x, y));
            let n = (x, y - 1);
            let s = (x, y + 1);
            let e = (x + 1, y);
            let w = (x - 1, y);
            let new_xy = match &map.map[y][x] {
                Part::Animal => break,
                Part::NS => {
                    if (last_x, last_y) == n {
                        s
                    } else if (last_x, last_y) == s {
                        n
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                Part::EW => {
                    if (last_x, last_y) == e {
                        w
                    } else if (last_x, last_y) == w {
                        e
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                Part::NE => {
                    if (last_x, last_y) == n {
                        e
                    } else if (last_x, last_y) == e {
                        n
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                Part::NW => {
                    if (last_x, last_y) == n {
                        w
                    } else if (last_x, last_y) == w {
                        n
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                Part::SW => {
                    if (last_x, last_y) == w {
                        s
                    } else if (last_x, last_y) == s {
                        w
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                Part::SE => {
                    if (last_x, last_y) == e {
                        s
                    } else if (last_x, last_y) == s {
                        e
                    } else {
                        i = 0;
                        dir += 1;
                        (x, y) = map.start;
                        last_x = x;
                        last_y = y;
                        (x, y)
                    }
                },
                _ => {
                    i = 0;
                    dir += 1;
                    (x, y) = map.start;
                    last_x = x;
                    last_y = y;
                    (x, y)
                }
            };
            (last_x, last_y) = (x, y);
            (x, y) = new_xy;
        }
        i += 1;
    }
    
    let dir = match dir {
        0 => Dir::S,
        1 => Dir::E,
        2 => Dir::W,
        3 => Dir::N,
        _ => panic!()
    };

    (i - 1, in_loop, dir)
}

fn parse(input: &Vec<String>) -> Map {
    let mut ret = vec![];
    let mut start = (0, 0);
    for (y, line) in input.iter().enumerate() {
        ret.push(vec![]);
        for (x, char) in line.chars().enumerate() {
            let part: Part = char.try_into().unwrap();
            if part == Part::Animal {
                start = (x, y);
            }
            ret[y].push(part);
        }
    }
    Map { map: ret, start }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    Animal,
    Ground,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl TryFrom<char> for Part {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Part::NS), //is a vertical pipe connecting north and south.
            '-' => Ok(Part::EW), //is a horizontal pipe connecting east and west.
            'L' => Ok(Part::NE), //is a 90-degree bend connecting north and east.
            'J' => Ok(Part::NW), //is a 90-degree bend connecting north and west.
            '7' => Ok(Part::SW), //is a 90-degree bend connecting south and west.
            'F' => Ok(Part::SE), //is a 90-degree bend connecting south and east.
            '.' => Ok(Part::Ground), //is ground; there is no pipe in this tile.
            'S' => Ok(Part::Animal), //is the starting position of the animal;
            _ => Err("Not a part.")
        }
    }
}

struct Map {
    map: Vec<Vec<Part>>,
    start: (usize, usize),
}