use std::{time::Instant, collections::HashSet};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let time = Instant::now();

    println!("Day 16");

    let input = read_input_to_vec("input/day16.txt");
    let map: Map = input.try_into().unwrap();

    let result = map.find_energized(Beam { x: 0, y: 0, direction: Direction::Right });
    println!("Part 1: {result}");

    let mut max = 0;
    for x in 0..map.0[0].len() {
        max = max.max(map.find_energized(Beam { x: x, y: 0, direction: Direction::Down }));
        max = max.max(map.find_energized(Beam { x: x, y: map.0.len() - 1, direction: Direction::Up }));
    }

    for y in 0..map.0.len() {
        max = max.max(map.find_energized(Beam { x: 0, y: y, direction: Direction::Right }));
        max = max.max(map.find_energized(Beam { x: map.0[0].len() - 1, y: y, direction: Direction::Left }));
    }

    println!("Part 2: {max}");

    println!("Time: {:?}", time.elapsed());
}

struct Map (Vec<Vec<Tile>>);

impl Map {
    fn find_energized(&self, start: Beam) -> usize {
        let mut beams = vec![start];
        let mut energized: HashSet<(usize, usize)> = HashSet::new();
        let mut loop_detection: HashSet<(usize, usize, Direction)> = HashSet::new();

        'outer: while let Some(beam) = beams.pop() {
            let mut beam = beam;
            loop {
                let tile = &self.0[beam.y][beam.x];
                match tile {
                    Tile::Empty => (),
                    Tile::LRSplitter => {
                        if beam.direction == Direction::Up || beam.direction == Direction::Down {
                            beam.direction = Direction::Left;
                            beams.push(Beam { x: beam.x, y: beam.y, direction: Direction::Right });
                        }
                    },
                    Tile::UDSplitter => {
                        if beam.direction == Direction::Left || beam.direction == Direction::Right {
                            beam.direction = Direction::Up;
                            beams.push(Beam { x: beam.x, y: beam.y, direction: Direction::Down });
                        }
                    },
                    Tile::LDRUMirror => {
                        beam.direction = match beam.direction {
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                        }
                    },
                    Tile::LURDMirror => {
                        beam.direction = match beam.direction {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                        }
                    },
                }
                energized.insert((beam.x, beam.y));
                if loop_detection.contains(&(beam.x, beam.y, beam.direction)) {
                    continue 'outer;
                } else {
                    loop_detection.insert((beam.x, beam.y, beam.direction));
                }

                //println!("Tile: {:?}, Direction: {:?}", tile, beam.direction);

                match beam.direction {
                    Direction::Left => {
                        if beam.x as i64 - 1 < 0 {
                            continue 'outer;
                        } else {
                            beam.x -= 1;
                        }
                    },
                    Direction::Right => {
                        if beam.x + 1 >= self.0[0].len() {
                            continue 'outer;
                        } else {
                            beam.x += 1;
                        }
                    },
                    Direction::Up => {
                        if beam.y as i64 - 1 < 0 {
                            continue 'outer;
                        } else {
                            beam.y -= 1;
                        }
                    },
                    Direction::Down => {
                        if beam.y + 1 >= self.0[0].len() {
                            continue 'outer;
                        } else {
                            beam.y += 1;
                        }
                    },
                }

            }
        }
        energized.len()
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut ret = vec![];

        for line in value {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c.try_into()?)
            }

            ret.push(row);
        }

        Ok(Map(ret))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}


#[derive(Debug)]
enum Tile {
    Empty,
    LRSplitter,
    UDSplitter,
    LDRUMirror,
    LURDMirror,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '-' => Ok(Tile::LRSplitter),
            '|' => Ok(Tile::UDSplitter),
            '/' => Ok(Tile::LURDMirror),
            '\\' => Ok(Tile::LDRUMirror),
            _ => Err(()),
        }
    }
}