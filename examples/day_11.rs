use std::{collections::HashMap, time::Instant};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let time = Instant::now();
    println!("Day 11");
    let input = read_input_to_vec("input/day11.txt");
    let mut space = parse(&input);
    let row_mod = get_expansion_modifier(&space.row_bucket, space.dim.1, 1);
    let col_mod = get_expansion_modifier(&space.col_bucket, space.dim.0, 1);
    let mut part1_space = space.clone();
    expand_space(&mut part1_space, row_mod, col_mod);

    let sum = dist_sum(&part1_space);

    println!("Part 1: {sum}");

    let row_mod = get_expansion_modifier(&space.row_bucket, space.dim.1, 999999);
    let col_mod = get_expansion_modifier(&space.col_bucket, space.dim.0, 999999);
    let mut part2_space = space.clone();
    expand_space(&mut part2_space, row_mod, col_mod);

    let sum = dist_sum(&part2_space);
    println!("Part 2: {sum}");
    println!("time: {:?}", time.elapsed());
}

fn dist_sum(space: &Space) -> usize {
    let mut sum = 0;
    for i in 0..space.galaxies.len() {
        for j in i + 1..space.galaxies.len() {
            let g1 = space.galaxies[i];
            let g2 = space.galaxies[j];
            let dist = calculate_manhattan_distance(&g1, &g2);
            //println!("{:?}, {:?} = {dist}", g1, g2);
            sum += dist;
        }
    }
    sum
}

fn calculate_manhattan_distance(g1: &Coordinate, g2: &Coordinate) -> usize {
    g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)
}

fn expand_space(input: &mut Space, row_mod: Vec<usize>, col_mod: Vec<usize>) {
    let new: Vec<Coordinate> = input
        .galaxies
        .iter()
        .map(|(x, y)| (x + col_mod[*x], y + row_mod[*y]))
        .collect();
    input.galaxies = new;
}

fn get_expansion_modifier(
    bucket: &HashMap<usize, u64>,
    length: usize,
    expansion: usize,
) -> Vec<usize> {
    let mut ret = vec![];
    let mut modifier = 0;
    for i in 0..length {
        match bucket.get(&i) {
            Some(_) => (),
            None => modifier += expansion,
        }
        ret.push(modifier);
    }
    ret
}

fn parse(input: &Vec<String>) -> Space {
    let mut galaxies = vec![];
    let mut row_bucket: HashMap<usize, u64> = HashMap::new();
    let mut col_bucket: HashMap<usize, u64> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                match row_bucket.get_mut(&y) {
                    Some(v) => *v += 1,
                    None => {
                        row_bucket.insert(y, 1);
                    }
                }
                match col_bucket.get_mut(&x) {
                    Some(v) => *v += 1,
                    None => {
                        col_bucket.insert(x, 1);
                    }
                }
            }
        }
    }
    Space {
        galaxies,
        row_bucket,
        col_bucket,
        dim: (input.len(), input[0].len()),
    }
}

type Coordinate = (usize, usize);

#[derive(Debug, Clone)]
struct Space {
    galaxies: Vec<Coordinate>,
    row_bucket: HashMap<usize, u64>,
    col_bucket: HashMap<usize, u64>,
    dim: (usize, usize),
}
