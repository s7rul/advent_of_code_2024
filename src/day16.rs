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

fn print_maze(maze: &[Vec<Tile>], visited: &HashSet<Position>, position: Position) {
    for (y, r) in maze.iter().enumerate() {
        for (x, t) in r.iter().enumerate() {
            if position == (Position { x, y }) {
                print!("R");
            } else if visited.contains(&Position { x, y }) {
                print!("x");
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

fn find_end(maze: &[Vec<Tile>], mut pos: Position, mut orientation: Orientation, mut visited: HashSet<(Position, Orientation)>, cache: &mut HashMap<(Position, Orientation), (bool, u32)>, fork_count: &mut u32) -> (bool, u32) {
    // find next junction
    let mut local_score = 0;

    if let Some(result) = cache.get(&(pos, orientation)) {
        return result.to_owned();
    }
    
    let pos_start = pos;
    let orientation_start = orientation;

    *fork_count += 1;
    //println!("fork count: {fork_count}");

    loop {
        if visited.contains(&(pos, orientation)) {
            return (false, 0);
        }
        visited.insert((pos, orientation));
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
                return (true, local_score + 1001)
            },
            (_, Tile::End, _) => {
                return (true, local_score + 1)
            },
            (_, _, Tile::End) => {
                return (true, local_score + 1001)
            },
            (Tile::Wall, Tile::Wall, Tile::Wall) => return (false, 0),
            (Tile::Empty, Tile::Wall, Tile::Wall) => {
                local_score += 1001;
                orientation.rotate_counterclockwise();
                pos.move_forward(orientation);
            }
            (Tile::Wall, Tile::Empty, Tile::Wall) => {
                local_score += 1;
                pos.move_forward(orientation);
            }
            (Tile::Wall, Tile::Wall, Tile::Empty) => {
                local_score += 1001;
                orientation.rotate_clockwise();
                pos.move_forward(orientation);
            }
            (vcc, vf, vc) => {
                if vf == Tile::Empty {
                    let mut pos_new = pos;
                    pos_new.move_forward(orientation);
                    let new = find_end(maze, pos_new, orientation,  visited.clone(), cache, fork_count);
                    possible.push((new.0, local_score + new.1 + 1));
                }

                if vcc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_counterclockwise();
                    pos_new.move_forward(orientation_new);
                    let new = find_end(maze, pos_new, orientation_new,  visited.clone(), cache, fork_count);
                    possible.push((new.0, local_score + new.1 + 1001));
                }

                if vc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_clockwise();
                    pos_new.move_forward(orientation_new);
                    let new = find_end(maze, pos_new, orientation_new,  visited.clone(), cache, fork_count);
                    possible.push((new.0, local_score + new.1 + 1001));
                }
            }
        }

        if !possible.is_empty() {
            let mut min = u32::MAX;
            let mut all_false = true;
            for (p, v) in possible {
                if p {
                    all_false = false;
                    min = min.min(v);
                }
            }
            let result = if all_false {
                (false, 0)
            } else {
                (true, min)
            };
            cache.insert((pos_start, orientation_start), result);
            return result;
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Map) -> u32 {
    let mut fork_count = 0;
    let mut visited = HashMap::new();
    find_end(&input.maze, input.start, Orientation::Right, HashSet::new(), &mut visited, &mut fork_count).1
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
