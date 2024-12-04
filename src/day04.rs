use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| {l.chars().collect()}).collect()
}

fn is_windown_xmas_forward(input: &[char]) -> bool {
    input[0] == 'X' && input[1] == 'M' && input[2] == 'A' && input[3] == 'S'
}

fn is_windown_xmas_backward(input: &[char]) -> bool {
    input[0] == 'S' && input[1] == 'A' && input[2] == 'M' && input[3] == 'X'
}


fn return_n_x(input: &[Vec<char>]) -> i32 {
    let mut sum = 0;
// M.M
// .A.
// S.S
    if input[0][0] == 'M' && input[0][2] == 'M' && input[1][1] == 'A' && input[2][0] == 'S' && input[2][2] == 'S' {
        sum += 1;
    }
// S.M
// .A.
// S.M
    if input[0][0] == 'S' && input[0][2] == 'M' && input[1][1] == 'A' && input[2][0] == 'S' && input[2][2] == 'M' {
        sum += 1;
    }
// S.S
// .A.
// M.M
    if input[0][0] == 'S' && input[0][2] == 'S' && input[1][1] == 'A' && input[2][0] == 'M' && input[2][2] == 'M' {
        sum += 1;
    }
// M.S
// .A.
// M.S
    if input[0][0] == 'M' && input[0][2] == 'S' && input[1][1] == 'A' && input[2][0] == 'M' && input[2][2] == 'S' {
        sum += 1;
    }

    sum
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> i32 {
    let mut sum = 0;
    for x in 0..(input[0].len() - 2) {
        for y in 0..(input.len() - 2) {
            let mut cube = vec![];
            for i in y..(y + 3) {
                let mut row = vec![];
                for j in x..(x + 3) {
                    row.push(input[i][j]);
                }
                cube.push(row);
            }
            sum += return_n_x(&cube);
        }
    }
    sum
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> i32 {
    let mut sum = 0;
    // search right
    for y in 0..input.len() {
        for x in 0..(input[0].len() - 3) {
            let window = &input[y][x..(x+4)];
            if is_windown_xmas_forward(window) {
                sum += 1;
            }
            if is_windown_xmas_backward(window) {
                sum += 1;
            }
        }
    }


    // search down
    for x in 0..input[0].len() {
        for y in 0..(input.len() - 3) {
            let mut window = vec![];
            window.push(input[y][x]);
            window.push(input[y + 1][x]);
            window.push(input[y + 2][x]);
            window.push(input[y + 3][x]);
            if is_windown_xmas_forward(&window) {
                sum += 1;
            }
            if is_windown_xmas_backward(&window) {
                sum += 1;
            }
        }
    }


    // diag 1
    for y in 0..(input.len() - 3) {
        for x in 0..(input[0].len() - 3) {
            let mut window = vec![];
            window.push(input[y][x]);
            window.push(input[y + 1][x + 1]);
            window.push(input[y + 2][x + 2]);
            window.push(input[y + 3][x + 3]);
            if is_windown_xmas_forward(&window) {
                sum += 1;
            }
            if is_windown_xmas_backward(&window) {
                sum += 1;
            }
        }
    }

    // diag 2
    for y in 0..(input.len() - 3) {
        for x in 3..input[0].len() {
            let mut window = vec![];
            window.push(input[y][x]);
            window.push(input[y + 1][x - 1]);
            window.push(input[y + 2][x - 2]);
            window.push(input[y + 3][x - 3]);
            if is_windown_xmas_forward(&window) {
                sum += 1;
            }
            if is_windown_xmas_backward(&window) {
                sum += 1;
            }
        }
    }

    sum
}


#[test]
fn test1_2() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let parsed = generator(input);
    let result = solve_part2(&parsed);
    assert_eq!(result, 9);
}

#[test]
fn test2_2() {
    let input = "..........
...M.M....
....A.....
...S.S....";
    let parsed = generator(input);
    let result = solve_part2(&parsed);
    assert_eq!(result, 1);
}
#[test]
fn test1() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let parsed = generator(input);
    let result = solve_part1(&parsed);
    assert_eq!(result, 18);
}

#[test]
fn test2() {
    let input = "XMAS......
......SAMX
...XMAS...";
    let parsed = generator(input);
    let result = solve_part1(&parsed);
    assert_eq!(result, 3);
}
#[test]
fn test3() {
    let input = "X.........
M.........
A...X.....
S...M....S
....A....A
....S....M
.........X";
    let parsed = generator(input);
    let result = solve_part1(&parsed);
    assert_eq!(result, 3);
}

//#[test]
fn test5() {
    let input = "X.........
.M........
..AX......
...SM.S...
.....A.A..
......S.M.
.........X";
    let parsed = generator(input);
    let result = solve_part1(&parsed);
    assert_eq!(result, 3);
}

#[test]
fn test6() {
    let input = "X.........
.M........
..A.......
...S......
..........
..........
..........";
    let parsed = generator(input);
    let result = solve_part1(&parsed);
    assert_eq!(result, 1);
}
