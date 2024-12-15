#![allow(clippy::comparison_chain)]
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
struct Robot {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Move {
    Up, Down, Left, Right
}

#[derive(Debug, Clone)]
pub struct Input {
    map: Vec<Vec<Tile>>,
    robot: Robot,
    moves: Vec<Move>
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile2 {
    BoxLeft,
    BoxRight,
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Input2 {
    map: Vec<Vec<Tile2>>,
    robot: Robot,
    moves: Vec<Move>
}

impl Input2 {
    fn print_map(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == self.robot.x && y == self.robot.y {
                    print!("@");
                } else {
                    match tile {
                        Tile2::BoxLeft => print!("["),
                        Tile2::BoxRight => print!("]"),
                        Tile2::Wall => print!("#"),
                        Tile2::Empty => print!("."),
                    }
                }
            }
            println!();
        }
    }
}

#[aoc_generator(day15, part2)]
pub fn generator2(input: &str) -> Input2 {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut robot = Robot { x: 0, y: 0};
    let mut map = vec![];
    for (y, l) in map_str.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    row.push(Tile2::Wall);
                    row.push(Tile2::Wall);
                },
                'O' => {
                    row.push(Tile2::BoxLeft);
                    row.push(Tile2::BoxRight);
                },
                '.' => {
                    row.push(Tile2::Empty);
                    row.push(Tile2::Empty);
                },
                '@' => {
                    robot.x = x * 2;
                    robot.y = y;
                    row.push(Tile2::Empty);
                    row.push(Tile2::Empty);
                },
                _ => panic!(),
            }
        }
        map.push(row);
    }

    let moves = moves_str.chars().filter(|c| *c != '\n').map(|c| {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!(),
        }
    }).collect();

    Input2 { map, robot, moves }
}

fn move_once(m: &Move, input: &mut Input2) {
    match m {
        Move::Up => {
            let mut boxes: Vec<Vec<(usize, usize)>> = vec![vec![]];
            match input.map[input.robot.y - 1][input.robot.x] {
                Tile2::BoxLeft => {
                    boxes[0].push((input.robot.x, input.robot.x + 1));
                },
                Tile2::BoxRight => {
                    boxes[0].push((input.robot.x - 1, input.robot.x));
                },
                Tile2::Wall => {
                    return;
                },
                Tile2::Empty => {
                    input.robot.y -= 1;
                    return;
                },
            }

            let mut n = 1;
            loop {
                let mut row = vec![];
                let mut all_empty = true;
                
                for box_x in &boxes[n - 1] {
                    match input.map[input.robot.y - n - 1][box_x.0] {
                        Tile2::Wall => return,
                        Tile2::Empty => (),
                        Tile2::BoxLeft => {
                            all_empty = false;
                            row.push((box_x.0, box_x.0 + 1));
                        },
                        Tile2::BoxRight => {
                            all_empty = false;
                            row.push((box_x.0 - 1, box_x.0));
                        },
                    }
                    match input.map[input.robot.y - n - 1][box_x.1] {
                        Tile2::Wall => return,
                        Tile2::Empty => (),
                        Tile2::BoxLeft => {
                            all_empty = false;
                            row.push((box_x.1, box_x.1 + 1));
                        },
                        Tile2::BoxRight => {
                            all_empty = false;
                            row.push((box_x.1 - 1, box_x.1));
                        },
                    }
                }
                boxes.push(row);
                if all_empty {
                    break;
                }
                n += 1;
            }

            for (n, row) in boxes.iter().enumerate().rev() {
                for x in row {
                    let y = input.robot.y - n - 1;
                    input.map[y][x.0] = Tile2::Empty;
                    input.map[y][x.1] = Tile2::Empty;
                    input.map[y - 1][x.0] = Tile2::BoxLeft;
                    input.map[y - 1][x.1] = Tile2::BoxRight;
                }
            }
            input.robot.y -= 1;
        },
        Move::Down => {
            let mut boxes: Vec<Vec<(usize, usize)>> = vec![vec![]];
            match input.map[input.robot.y + 1][input.robot.x] {
                Tile2::BoxLeft => {
                    boxes[0].push((input.robot.x, input.robot.x + 1));
                },
                Tile2::BoxRight => {
                    boxes[0].push((input.robot.x - 1, input.robot.x));
                },
                Tile2::Wall => {
                    return;
                },
                Tile2::Empty => {
                    input.robot.y += 1;
                    return;
                },
            }

            let mut n = 1;
            loop {
                let mut row = vec![];
                let mut all_empty = true;
                
                for box_x in &boxes[n - 1] {
                    match input.map[input.robot.y + n + 1][box_x.0] {
                        Tile2::Wall => return,
                        Tile2::Empty => (),
                        Tile2::BoxLeft => {
                            all_empty = false;
                            row.push((box_x.0, box_x.0 + 1));
                        },
                        Tile2::BoxRight => {
                            all_empty = false;
                            row.push((box_x.0 - 1, box_x.0));
                        },
                    }
                    match input.map[input.robot.y + n + 1][box_x.1] {
                        Tile2::Wall => return,
                        Tile2::Empty => (),
                        Tile2::BoxLeft => {
                            all_empty = false;
                            row.push((box_x.1, box_x.1 + 1));
                        },
                        Tile2::BoxRight => {
                            all_empty = false;
                            row.push((box_x.1 - 1, box_x.1));
                        },
                    }
                }
                boxes.push(row);
                if all_empty {
                    break;
                }
                n += 1;
            }

            for (n, row) in boxes.iter().enumerate().rev() {
                for x in row {
                    let y = input.robot.y + n + 1;
                    input.map[y][x.0] = Tile2::Empty;
                    input.map[y][x.1] = Tile2::Empty;
                    input.map[y + 1][x.0] = Tile2::BoxLeft;
                    input.map[y + 1][x.1] = Tile2::BoxRight;
                }
            }
            input.robot.y += 1;
        },
        Move::Left => {
            let mut n = 1;
            while input.map[input.robot.y][input.robot.x - n] == Tile2::BoxRight {
                n += 2;
            }
            
            match input.map[input.robot.y][input.robot.x - n] {
                Tile2::Wall => (),
                Tile2::Empty => {
                    if n != 1 {
                        for (i, x) in (input.robot.x - n..input.robot.x - 1).enumerate() {
                            if i % 2 == 0 {
                                input.map[input.robot.y][x] = Tile2::BoxLeft;
                            } else {
                                input.map[input.robot.y][x] = Tile2::BoxRight;
                            }
                        }
                        input.map[input.robot.y][input.robot.x - 1] = Tile2::Empty;
                    }
                    input.robot.x -= 1;
                },
                _ => panic!(),
            }
        },
        Move::Right => {
            let mut n = 1;
            while input.map[input.robot.y][input.robot.x + n] == Tile2::BoxLeft {
                n += 2;
            }
            
            match input.map[input.robot.y][input.robot.x + n] {
                Tile2::Wall => (),
                Tile2::Empty => {
                    if n != 1 {
                        for (i, x) in (input.robot.x + 2..= input.robot.x + n).enumerate() {
                            if i % 2 == 0 {
                                input.map[input.robot.y][x] = Tile2::BoxLeft;
                            } else {
                                input.map[input.robot.y][x] = Tile2::BoxRight;
                            }
                        }
                        input.map[input.robot.y][input.robot.x + 1] = Tile2::Empty;
                    }
                    input.robot.x += 1;
                },
                _ => {
                    input.print_map();
                    panic!()
                },
            }
        },
    }
}

#[test]
fn test_2_1() {
    let mut input = generator2("#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^");
    for m in input.moves.clone() {
        move_once(&m, &mut input);
        input.print_map();
    }
    //panic!()
}

#[test]
fn test_2_3() {
    let mut input = generator2("#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

^<<<v");
    for m in input.moves.clone() {
        move_once(&m, &mut input);
        input.print_map();
    }
    panic!()
}

#[test]
fn test_2_2() {
    let mut input = generator2("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
    input.print_map();
    for m in input.moves.clone() {
        move_once(&m, &mut input);
        println!("move: {:?}", m);
        input.print_map();
    }
    //panic!()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Input2) -> usize {
    let mut input = input.clone();

    for m in &input.moves.clone() {
        move_once(m, &mut input)
    }

    let mut sum = 0;
    for (y, row) in input.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile2::BoxLeft {
                sum += y * 100 + x;
            }
        }
    }
    sum
}

#[aoc_generator(day15, part1)]
pub fn generator(input: &str) -> Input {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut robot = Robot { x: 0, y: 0};
    let map = map_str.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            match c {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '.' => Tile::Empty,
                '@' => {
                    robot.x = x;
                    robot.y = y;
                    Tile::Empty
                },
                _ => panic!(),
            }
        }).collect()
    }).collect();

    let moves = moves_str.chars().filter(|c| *c != '\n').map(|c| {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!(),
        }
    }).collect();

    Input { map, robot, moves }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let mut input = input.clone();

    for m in input.moves {
        match m {
            Move::Up => {
                let mut n = 1;
                while input.map[input.robot.y - n][input.robot.x] == Tile::Box {
                    n += 1;
                }
                
                match input.map[input.robot.y - n][input.robot.x] {
                    Tile::Wall => (),
                    Tile::Empty => {
                        if n != 1 {
                            input.map[input.robot.y - n][input.robot.x] = Tile::Box;
                            input.map[input.robot.y - 1][input.robot.x] = Tile::Empty;
                        }
                        input.robot.y -= 1;
                    },
                    Tile::Box => panic!(),
                }
            },
            Move::Down => {
                let mut n = 1;
                while input.map[input.robot.y + n][input.robot.x] == Tile::Box {
                    n += 1;
                }
                
                match input.map[input.robot.y + n][input.robot.x] {
                    Tile::Wall => (),
                    Tile::Empty => {
                        if n != 1 {
                            input.map[input.robot.y + n][input.robot.x] = Tile::Box;
                            input.map[input.robot.y + 1][input.robot.x] = Tile::Empty;
                        }
                        input.robot.y += 1;
                    },
                    Tile::Box => panic!(),
                }
            },
            Move::Left => {
                let mut n = 1;
                while input.map[input.robot.y][input.robot.x - n] == Tile::Box {
                    n += 1;
                }
                
                match input.map[input.robot.y][input.robot.x - n] {
                    Tile::Wall => (),
                    Tile::Empty => {
                        if n != 1 {
                            input.map[input.robot.y][input.robot.x - n] = Tile::Box;
                            input.map[input.robot.y][input.robot.x - 1] = Tile::Empty;
                        }
                        input.robot.x -= 1;
                    },
                    Tile::Box => panic!(),
                }
            },
            Move::Right => {
                let mut n = 1;
                while input.map[input.robot.y][input.robot.x + n] == Tile::Box {
                    n += 1;
                }
                
                match input.map[input.robot.y][input.robot.x + n] {
                    Tile::Wall => (),
                    Tile::Empty => {
                        if n != 1 {
                            input.map[input.robot.y][input.robot.x + n] = Tile::Box;
                            input.map[input.robot.y][input.robot.x + 1] = Tile::Empty;
                        }
                        input.robot.x += 1;
                    },
                    Tile::Box => panic!(),
                }
            },
        }
    }

    let mut sum = 0;
    for (y, row) in input.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Box {
                sum += y * 100 + x;
            }
        }
    }
    sum
}
