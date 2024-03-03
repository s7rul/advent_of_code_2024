use std::time::Instant;

use advent_of_code_2022::read_input_file;

fn main() {
    let time = Instant::now();
    println!("Day 13");

    let input = read_input_file("input/day13.txt");
    let patterns: Vec<Pattern> = input
        .split("\n\n")
        .map(|p| p.lines().collect::<Vec<&str>>().try_into().unwrap())
        .collect();

    let sum = (&patterns).iter().fold(0, |acc, p| {
        acc + match p.find_refection() {
            Reflection::Row(_, x) => x,
            Reflection::Col(_, y) => y * 100,
        } as u128
    });

    println!("Part 1: {sum}");

    let sum = (&patterns).iter().fold(0, |acc, p| {
        acc + match p.find_refection_with_smudge() {
            Reflection::Row(_, x) => x,
            Reflection::Col(_, y) => y * 100,
        } as u128
    });

    println!("Part 2: {sum}");

    println!("time: {:?}", time.elapsed());
}

#[derive(Debug)]
enum Reflection {
    Row(usize, usize),
    Col(usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Ash),
            '#' => Ok(Tile::Rock),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Pattern {
    map: Vec<Vec<Tile>>,
}

fn check_row(row: &Vec<Tile>, i: usize) -> bool {
    for x in 0..=i {
        let to_check = i + ((i + 1) - x);
        if to_check >= row.len() {
            continue;
        }
        if row[x] != row[to_check] {
            return false;
        }
    }
    true
}

fn count_errors(row: &Vec<Tile>, i: usize) -> u128 {
    let mut errors = 0;
    for x in 0..=i {
        let to_check = i + ((i + 1) - x);
        if to_check >= row.len() {
            continue;
        }
        if row[x] != row[to_check] {
            errors += 1;
        }
    }
    errors
}

impl Pattern {
    fn find_refection(&self) -> Reflection {
        // check rows

        'outer_row: for x in 0..self.map[0].len() - 1 {
            for row in &self.map {
                if !check_row(row, x) {
                    continue 'outer_row;
                }
            }

            return Reflection::Row(x, x + 1);
        }

        let mut cols = vec![];
        for x in 0..self.map[0].len() {
            let mut col = vec![];
            for y in 0..self.map.len() {
                col.push(self.map[y][x]);
            }
            cols.push(col);
        }

        'outer_col: for y in 0..cols[0].len() - 1 {
            for col in &cols {
                if !check_row(col, y) {
                    continue 'outer_col;
                }
            }

            return Reflection::Col(y, y + 1);
        }
        panic!()
    }

    fn find_refection_with_smudge(&self) -> Reflection {
        // check rows

        'outer_row: for x in 0..self.map[0].len() - 1 {
            let mut errors = 0;
            for row in &self.map {
                errors += count_errors(row, x);
                if errors > 1 {
                    continue 'outer_row;
                }
            }

            if errors == 1 {
                return Reflection::Row(x, x + 1);
            }
        }

        let mut cols = vec![];
        for x in 0..self.map[0].len() {
            let mut col = vec![];
            for y in 0..self.map.len() {
                col.push(self.map[y][x]);
            }
            cols.push(col);
        }

        'outer_col: for y in 0..cols[0].len() - 1 {
            let mut errors = 0;
            for col in &cols {
                errors += count_errors(col, y);
                if errors > 1 {
                    continue 'outer_col;
                }
            }

            if errors == 1 {
                return Reflection::Col(y, y + 1);
            }
        }
        panic!()
    }
}

impl TryFrom<Vec<&str>> for Pattern {
    type Error = ();

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let mut ret = vec![];

        for line in value {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c.try_into()?);
            }

            ret.push(row);
        }

        Ok(Pattern { map: ret })
    }
}
