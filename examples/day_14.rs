use std::{collections::HashMap, fmt::Display, time::Instant};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let time = Instant::now();
    println!("Day 14");
    let input = read_input_to_vec("input/day14.txt");
    let mut map: Map = input.try_into().unwrap();
    let mut map2 = map.clone();
    while map.tilt_north() {}

    let load = map.calculate_load();
    println!("Part 1: {}", load);
    let mut hash: HashMap<Map, usize> = HashMap::new();

    let mut i = 0;
    let mut not_detected = true;
    while i < 1000000000 {
        if not_detected {
            match hash.get(&map2) {
                Some(v) => {
                    println!("cycle detected");
                    let length = i - v;
                    let reminder = 1000000000 - i;
                    let loops = reminder / length;
                    i += loops * length;
                    not_detected = false;
                }
                None => {
                    hash.insert(map2.clone(), i);
                    map2.cycle();
                    i += 1;
                }
            }
        } else {
            map2.cycle();
            i += 1;
        }
    }

    let load = map2.calculate_load();
    println!("Part 2: {}", load);

    println!("Time: {:?}", time.elapsed());
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Tile {
    Ground,
    Round,
    Cube,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Ground),
            '#' => Ok(Tile::Cube),
            'O' => Ok(Tile::Round),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Round => write!(f, "O"),
            Tile::Cube => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn tilt_north(&mut self) -> bool {
        let mut has_moved = false;
        let map = &mut self.0;

        for y in 1..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == Tile::Round {
                    if map[y - 1][x] == Tile::Ground {
                        map[y][x] = Tile::Ground;
                        map[y - 1][x] = Tile::Round;
                        has_moved = true;
                    }
                }
            }
        }

        has_moved
    }

    fn tilt_west(&mut self) -> bool {
        let mut has_moved = false;
        let map = &mut self.0;

        for x in 1..map[0].len() {
            for y in 0..map.len() {
                if map[y][x] == Tile::Round {
                    if map[y][x - 1] == Tile::Ground {
                        map[y][x] = Tile::Ground;
                        map[y][x - 1] = Tile::Round;
                        has_moved = true;
                    }
                }
            }
        }

        has_moved
    }

    fn tilt_east(&mut self) -> bool {
        let mut has_moved = false;
        let map = &mut self.0;

        for x in (0..map[0].len() - 1).rev() {
            for y in 0..map.len() {
                if map[y][x] == Tile::Round {
                    if map[y][x + 1] == Tile::Ground {
                        map[y][x] = Tile::Ground;
                        map[y][x + 1] = Tile::Round;
                        has_moved = true;
                    }
                }
            }
        }

        has_moved
    }

    fn tilt_south(&mut self) -> bool {
        let mut has_moved = false;
        let map = &mut self.0;

        for y in (0..map.len() - 1).rev() {
            for x in 0..map[0].len() {
                if map[y][x] == Tile::Round {
                    if map[y + 1][x] == Tile::Ground {
                        map[y][x] = Tile::Ground;
                        map[y + 1][x] = Tile::Round;
                        has_moved = true;
                    }
                }
            }
        }

        has_moved
    }

    fn cycle(&mut self) {
        while self.tilt_north() {}
        while self.tilt_west() {}
        while self.tilt_south() {}
        while self.tilt_east() {}
    }

    fn calculate_load(&self) -> u64 {
        let map = &self.0;
        let mut sum = 0;
        for x in 0..map[0].len() {
            for y in 0..map.len() {
                if map[y][x] == Tile::Round {
                    sum += (map.len() - y) as u64;
                }
            }
        }
        sum
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

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f);
        for line in &self.0 {
            for tile in line {
                write!(f, "{}", tile);
            }
            writeln!(f);
        }

        Ok(())
    }
}
