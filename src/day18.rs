#![allow(clippy::comparison_chain)]
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty, Corrupted
}

// 

pub struct Point {
    x: usize,
    y: usize,
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Point> {
    input.lines().map(|l| {
        let (x_str, y_str) = l.split_once(',').unwrap();
        Point { x: x_str.parse().unwrap(), y: y_str.parse().unwrap() }
    }).collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

struct ListNode {
    score: u32,
    position: Position,
}

struct SortedNodeList {
    list: Vec<ListNode>
}

impl SortedNodeList {
    fn new() -> Self {
        Self { list: vec![] }
    }

    fn get_next(&mut self) -> Option<ListNode> {
        self.list.pop()
    }

    fn find(&mut self, item: &ListNode) -> Option<(usize, &mut ListNode)> {
        for (i, n) in self.list.iter_mut().enumerate() {
            if n.position == item.position {
                return Some((i, n));
            }
        }
        None
    }

    fn insert(&mut self, item: ListNode) {
        if let Some((i, node)) = self.find(&item) {
            if node.score > item.score {
                self.list.remove(i);
            } else {
                return;
            }
        }

        for (i, n) in self.list.iter().enumerate() {
            if item.score > n.score {
                self.list.insert(i, item);
                return;
            }
        }
        self.list.push(item);
    }
}

fn find_shortest(mem_map: &[Vec<Tile>]) -> Option<u32> {
    let start = Position {x: 0, y: 0};
    let largest_x = mem_map[0].len() - 1;
    let largest_y = mem_map.len() - 1;
    let end = Position {x: largest_x, y: largest_y};
    let mut visited: HashSet<Position> = HashSet::new();
    let mut next = SortedNodeList::new();

    next.insert(ListNode { score: 0, position: start });

    loop {
        let next_node = match next.get_next() {
            Some(n) => n,
            None => return None,
        };
        visited.insert(next_node.position);

        if next_node.position == end {
            return Some(next_node.score);
        }

        let mut potential = vec![];

        // up
        if next_node.position.y > 0 {
            potential.push(Position {x: next_node.position.x, y: next_node.position.y - 1});
        }
        // down
        if next_node.position.y < largest_y{
            potential.push(Position {x: next_node.position.x, y: next_node.position.y + 1});
        }
        // left
        if next_node.position.x > 0 {
            potential.push(Position {x: next_node.position.x - 1, y: next_node.position.y});
        }
        // right
        if next_node.position.x < largest_x {
            potential.push(Position {x: next_node.position.x + 1, y: next_node.position.y});
        }

        for p in potential {
            if !visited.contains(&p) && mem_map[p.y][p.x] != Tile::Corrupted {
                next.insert(ListNode { score: next_node.score + 1, position: p });
            }
        }
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Point]) -> u32 {
    let mut memmory = vec![vec![Tile::Empty; 71]; 71];

    for p in input.iter().take(1024) {
        memmory[p.y][p.x] = Tile::Corrupted;
    }

    find_shortest(&memmory).unwrap()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Point]) -> String {
    let mut memmory = vec![vec![Tile::Empty; 71]; 71];

    for p in input {
        memmory[p.y][p.x] = Tile::Corrupted;

        match find_shortest(&memmory) {
            Some(_) => (),
            None => {
                return format!("{},{}", p.x, p.y);
            },
        }
    }
    "".to_string()
}
