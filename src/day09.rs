#![allow(clippy::comparison_chain)]
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - b'0').collect()
}

#[derive(Debug)]
enum Block {
    Empty,
    File(u64),
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u8]) -> u64 {
    let mut file_layout = vec![];
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*n {
                file_layout.push(Block::File((i / 2) as u64));
            }
        } else {
            for _ in 0..*n {
                file_layout.push(Block::Empty);
            }
        }
    }

    let mut i = 0;
    let mut j = file_layout.len() - 1;
    while i != j {
        let target = &file_layout[i];
        match target {
            Block::Empty => {
                loop {
                    if let Block::File(id) = &file_layout[j] {
                        file_layout[i] = Block::File(*id);
                        file_layout[j] = Block::Empty;
                        j -= 1;
                        break;
                    }
                    j -= 1;
                }
            },
            Block::File(_) => (),
        }
        i += 1;
    }

    let mut sum = 0;

    for (i, b) in file_layout.iter().enumerate() {
        if let Block::File(v) = b {
            sum += i as u64 * v;
        } else {
            break;
        }
    }

    sum
}

#[derive(Debug)]
enum Space {
    Empty(u64),
    File(u64, u64),
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u8]) -> u64 {
    let mut file_layout = vec![];
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            file_layout.push(Space::File(*n as u64, (i / 2) as u64));
        } else {
            file_layout.push(Space::Empty(*n as u64));
        }
    }

    println!("layout: {:?}", file_layout);

    let mut i: isize = file_layout.len() as isize - 1;
    'out: while i >= 0 {
        if let Space::File(f_size, id) = file_layout[i as usize] {
            let mut j = 0;
            while j < i {
                if let Space::Empty(e_size) = file_layout[j as usize] {
                    if e_size > f_size {
                        file_layout[i as usize] = Space::Empty(f_size);
                        file_layout[j as usize] = Space::Empty(e_size - f_size);
                        file_layout.insert(j as usize, Space::File(f_size, id));
                        continue 'out;
                    } else if e_size == f_size {
                        file_layout[i as usize] = Space::Empty(f_size);
                        file_layout[j as usize] = Space::File(f_size, id);
                        break;
                    }
                }
                j += 1;
            }
        }
        i -= 1;
    }

    println!("layout: {:?}", file_layout);

    let mut sum = 0;
    let mut pos = 0;
    for n in file_layout {
        match n {
            Space::Empty(s) => pos += s,
            Space::File(s, id) => {
                for _ in 0..s {
                    println!("pos: {}, id: {}", pos, id);
                    sum += pos * id;
                    pos += 1;
                }
            },
        }
    }

    sum
}

#[test]
fn test_part2_1() {
    let input = generator("2333133121414131402");
    let result = solve_part2(&input);
    assert_eq!(2858, result);
}
