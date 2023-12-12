use std::time::Instant;

use advent_of_code_2022::read_input_to_vec;

fn main() {
    println!("day 1");

    let start = Instant::now();
    let input = read_input_to_vec("input/day1.txt");

    let mut sum = 0u64;

    for line in &input {
        let mut first: Option<u64> = None;
        let mut last: Option<u64> = None;

        for c in line.chars() {
            if c.is_digit(10) {
                match first {
                    Some(_) => last = Some(c as u64 - '0' as u64),
                    None => {
                        first = Some(c as u64 - '0' as u64);
                        last = Some(c as u64 - '0' as u64)
                    }
                }
            }
        }

        let number = first.unwrap() * 10 + last.unwrap();
        sum += number;
    }

    println!("Part1: {}", sum);

    let mut sum = 0u64;

    for line in &input {
        let mut first: Option<u64> = None;
        let mut last: Option<u64> = None;

        let chars: Vec<char> = (&line).chars().collect();

        for i in 0..line.len() {
            if chars[i].is_digit(10) {
                let c = chars[i];
                match first {
                    Some(_) => last = Some(c as u64 - '0' as u64),
                    None => {
                        first = Some(c as u64 - '0' as u64);
                        last = Some(c as u64 - '0' as u64)
                    }
                }
            } else {
                for j in (i + 2)..(i + 5).min(chars.len()) {
                    match string_to_number(&line[i..=j]) {
                        Some(v) => match first {
                            Some(_) => last = Some(v),
                            None => {
                                first = Some(v);
                                last = Some(v)
                            }
                        },
                        None => (),
                    }
                }
            }
        }

        let number = first.unwrap() * 10 + last.unwrap();
        sum += number;
    }

    println!("Part2: {sum}");
    println!("time: {:?}", start.elapsed());
}

fn string_to_number(input: &str) -> Option<u64> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
