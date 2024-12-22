#![allow(clippy::comparison_chain)]
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty, Wall, End
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Up, Down, Left, Right
}

impl Orientation {
    fn rotate_clockwise(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        };
    }

    fn rotate_counterclockwise(&mut self) {
        *self = match self {
            Orientation::Up => Orientation::Left,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Right => Orientation::Up,
        };
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_forward(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Up => self.y -= 1,
            Orientation::Down => self.y += 1,
            Orientation::Left => self.x -= 1,
            Orientation::Right => self.x += 1,
        };
    }
}

pub struct Map {
    maze: Vec<Vec<Tile>>,
    start: Position,
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Map {
    let mut start = Position {x: 0, y: 0};
    let maze = input.lines().enumerate().map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'E' => Tile::End,
            'S' => {
                start.x = x;
                start.y = y;
                Tile::Empty
            },
            _ => panic!(),
        }
    }).collect()).collect();
    Map { maze, start }
}

fn print_maze(maze: &[Vec<Tile>], visited: &HashSet<(Position, Orientation)>, position: Position) {
    for (y, r) in maze.iter().enumerate() {
        for (x, t) in r.iter().enumerate() {
            if position == (Position { x, y }) {
                print!("R");
            } else if visited.contains(&(Position { x, y }, Orientation::Up)) {
                print!("^");
            } else if visited.contains(&(Position { x, y }, Orientation::Down)) {
                print!("v");
            } else if visited.contains(&(Position { x, y }, Orientation::Left)) {
                print!("<");
            } else if visited.contains(&(Position { x, y }, Orientation::Right)) {
                print!(">");
            } else {
                match t {
                    Tile::Empty => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::End => print!("E"),
                }
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PathResult {
    Found, DeadEnd, Loop
}

fn find_end(maze: &[Vec<Tile>], mut pos: Position, mut orientation: Orientation, mut visited: HashSet<Position>, best_min: &mut u32, mut score: u32) -> Option<u32> {
    loop {
        if score > *best_min {
            println!("to long");
            return None;
        }

        if visited.contains(&pos) {
            return None;
        }
        visited.insert(pos);
        //print_maze(maze, &visited, pos);
        // look around
        // counterclokwise, forward, clockwise
        let around = match orientation {
            Orientation::Up => (maze[pos.y][pos.x - 1], maze[pos.y - 1][pos.x], maze[pos.y][pos.x +1]),
            Orientation::Down => (maze[pos.y][pos.x + 1], maze[pos.y + 1][pos.x], maze[pos.y][pos.x - 1]),
            Orientation::Left => (maze[pos.y + 1][pos.x], maze[pos.y][pos.x - 1], maze[pos.y - 1][pos.x]),
            Orientation::Right => (maze[pos.y - 1][pos.x], maze[pos.y][pos.x + 1], maze[pos.y + 1][pos.x]),
        };

        let mut possible = vec![];
        match around {
            (Tile::End, _, _) => {
                score += 1001;
                *best_min = (*best_min).min(score);
                println!("found end score: {score}");
                return Some(score)
            },
            (_, Tile::End, _) => {
                score += 1;
                *best_min = (*best_min).min(score);
                println!("found end score: {score}");
                return Some(score)
            },
            (_, _, Tile::End) => {
                score += 1001;
                *best_min = (*best_min).min(score);
                println!("found end score: {score}");
                return Some(score)
            },
            (Tile::Wall, Tile::Wall, Tile::Wall) => return None,
            (Tile::Empty, Tile::Wall, Tile::Wall) => {
                score += 1001;
                orientation.rotate_counterclockwise();
                pos.move_forward(orientation);
            }
            (Tile::Wall, Tile::Empty, Tile::Wall) => {
                score += 1;
                pos.move_forward(orientation);
            }
            (Tile::Wall, Tile::Wall, Tile::Empty) => {
                score += 1001;
                orientation.rotate_clockwise();
                pos.move_forward(orientation);
            }
            (vcc, vf, vc) => {
                if vf == Tile::Empty {
                    let mut pos_new = pos;
                    pos_new.move_forward(orientation);
                    let new = find_end(maze, pos_new, orientation,  visited.clone(), best_min, score + 1);
                    possible.push(new);
                }
                if vcc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_counterclockwise();
                    pos_new.move_forward(orientation_new);
                    let new = find_end(maze, pos_new, orientation_new,  visited.clone(), best_min, score + 1001);
                    possible.push(new);
                }
                if vc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_clockwise();
                    pos_new.move_forward(orientation_new);
                    let new = find_end(maze, pos_new, orientation_new,  visited.clone(), best_min, score + 1001);
                    possible.push(new);
                }
            }
        }

        if !possible.is_empty() {
            let mut min = u32::MAX;
            let mut all_false = true;
            for p in possible {
                if let Some(v) = p{
                    all_false = false;
                    min = min.min(v);
                }
            }
            let result = if all_false {
                None
            } else {
                Some(min)
            };
            return result;
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Map) -> u32 {
    let mut fork_count = u32::MAX;;
    
    find_end(&input.maze, input.start, Orientation::Right, HashSet::new(), &mut fork_count, 0).unwrap()
}

#[test]
fn test_1_1() {
    let input = generator("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(7036, reusult);
}

#[test]
fn test_1_2() {
    let input = generator("#######
#....E#
#.#####
#.....#
#S###.#
#######");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(2007, reusult);
}

#[test]
fn test_1_3() {
    let input = generator("#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(11048, reusult);
}

#[test]
fn test_1_4() {
    let input = generator("#######
#....E#
#.#####
#.....#
#S..#.#
#.....#
#######");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(2007, reusult);
}

#[test]
fn test_1_5() {
    let input = generator("###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(21148, reusult);
}

#[test]
fn test_1_6() {
    let input = generator("####################################################
#......................................#..........E#
#......................................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.................#...........#
#....................#.............................#
#S...................#.............................#
####################################################");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(5078, reusult);
}

#[test]
fn test_1_7() {
    let input = generator(
"####################
#...........#.....E#
#...........#......#
#.....#.....#......#
#.....#.....#......#
#.....#.....#......#
#.....#............#
#S....#............#
####################");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(5031, reusult);
}

#[test]
fn test_1_8() {
    let input = generator(
"#########
#.......#
#.......#
#...#...#
#...#...#
#S..#..E#
#########");
    println!("start: {:?}", input.start);
    let reusult = solve_part1(&input);
    assert_eq!(3012, reusult);
}
