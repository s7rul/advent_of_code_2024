#![allow(clippy::comparison_chain)]
use std::collections::HashSet;

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

struct ListNode {
    score: u32,
    position: Position,
    orientation: Orientation,
    visited: HashSet<Position>,
}

struct SortedNodeList {
    list: Vec<ListNode>
}

impl SortedNodeList {
    fn new() -> Self {
        Self { list: vec![] }
    }

    fn get_next(&mut self) -> ListNode {
        self.list.pop().unwrap()
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
            } else if node.score == item.score {
                node.visited = node.visited.union(&item.visited).copied().collect();
                return;
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




fn find_end(maze: &[Vec<Tile>], pos: Position, orientation: Orientation) -> ListNode {
    let mut next_list = SortedNodeList::new();
    next_list.insert(ListNode { score: 0, position: pos, orientation, visited: HashSet::new()});

    loop {
        let mut next_node = next_list.get_next();
        let score = next_node.score;
        let pos = next_node.position;
        let orientation = next_node.orientation;

        next_node.visited.insert(pos);

        let around = match orientation {
            Orientation::Up => (maze[pos.y][pos.x - 1], maze[pos.y - 1][pos.x], maze[pos.y][pos.x +1]),
            Orientation::Down => (maze[pos.y][pos.x + 1], maze[pos.y + 1][pos.x], maze[pos.y][pos.x - 1]),
            Orientation::Left => (maze[pos.y + 1][pos.x], maze[pos.y][pos.x - 1], maze[pos.y - 1][pos.x]),
            Orientation::Right => (maze[pos.y - 1][pos.x], maze[pos.y][pos.x + 1], maze[pos.y + 1][pos.x]),
        };

        match around {
            (Tile::End, _, _) => {
                let mut pos_new = pos;
                let mut orientation_new = orientation;
                orientation_new.rotate_counterclockwise();
                pos_new.move_forward(orientation_new);
                next_node.visited.insert(pos_new);
                next_node.score += 1001;
                return next_node;
            },
            (_, Tile::End, _) => {
                let mut pos_new = pos;
                pos_new.move_forward(orientation);
                next_node.visited.insert(pos_new);
                next_node.score += 1;
                return next_node;
            },
            (_, _, Tile::End) => {
                let mut pos_new = pos;
                let mut orientation_new = orientation;
                orientation_new.rotate_counterclockwise();
                pos_new.move_forward(orientation_new);
                next_node.visited.insert(pos_new);
                next_node.score += 1001;
                return next_node;
            },
            (Tile::Wall, Tile::Wall, Tile::Wall) => (),
            (vcc, vf, vc) => {
                if vf == Tile::Empty {
                    let mut pos_new = pos;
                    pos_new.move_forward(orientation);
                    next_list.insert(ListNode { score: score + 1, position: pos_new, orientation, visited: next_node.visited.clone() });
                }
                if vcc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_counterclockwise();
                    pos_new.move_forward(orientation_new);
                    next_list.insert(ListNode { score: score + 1001, position: pos_new, orientation: orientation_new, visited: next_node.visited.clone() });
                }
                if vc == Tile::Empty {
                    let mut pos_new = pos;
                    let mut orientation_new = orientation;
                    orientation_new.rotate_clockwise();
                    pos_new.move_forward(orientation_new);
                    next_list.insert(ListNode { score: score + 1001, position: pos_new, orientation: orientation_new, visited: next_node.visited.clone() });
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Map) -> u32 {
    find_end(&input.maze, input.start, Orientation::Right).score
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Map) -> usize {
    find_end(&input.maze, input.start, Orientation::Right).visited.len()
}

#[test]
fn test_2_1() {
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
    let reusult = solve_part2(&input);
    assert_eq!(45, reusult);
}

#[test]
fn test_2_2() {
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
    let reusult = solve_part2(&input);
    assert_eq!(64, reusult);
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

//#[test]
//fn test_1_6() {
//    let input = generator("####################################################
//#......................................#..........E#
//#......................................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.................#...........#
//#....................#.............................#
//#S...................#.............................#
//####################################################");
//    println!("start: {:?}", input.start);
//    let reusult = solve_part1(&input);
//    assert_eq!(5078, reusult);
//}

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
