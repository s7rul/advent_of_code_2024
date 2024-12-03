use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

fn is_valid_mul_instruction(input: &str) -> bool {
    let re = Regex::new(r"^mul\(\d{1,3},\d{1,3}\)$").unwrap();
    re.is_match(input)
}
fn is_valid_do_instruction(input: &str) -> bool {
    let re = Regex::new(r"^do\(\)$").unwrap();
    re.is_match(input)
}

fn is_valid_dont_instruction(input: &str) -> bool {
    let re = Regex::new(r"^don't\(\)$").unwrap();
    re.is_match(input)
}

#[derive(Debug)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}


#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Instruction> {
    let min_length = 3;
    let max_length = 13;

    let mut ret = vec![];

    let mut start = 0;
    while start < input.len() {
        for end in (start + min_length)..=(start + max_length) {
            if end >= input.len() {
                break;
            }
            let consider = &input[start..=end];
            //println!("start: {}, end: {}, consider: {}", start, end, consider);
            if is_valid_mul_instruction(consider) {
                let ist = &input[start..=end];
                let mid = &ist[4..(ist.len() - 1)];
                let nums: Vec<i32> = mid.split(',').map(|d| d.parse().unwrap()).collect();
                ret.push(Instruction::Mul(nums[0], nums[1]));
                //println!("found mul");
                break;
            } else if is_valid_do_instruction(consider) {
                ret.push(Instruction::Do);
                //println!("found do");
                break;
            } else if is_valid_dont_instruction(consider) {
                ret.push(Instruction::Dont);
                //println!("found don't");
                break;
            }
        }
        start += 1;
    }

    ret
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    let mut sum = 0;
    let mut enable = true;
    for ist in input {
        match ist {
            Instruction::Mul(x, y) => if enable {sum += x * y},
            Instruction::Do => enable = true,
            Instruction::Dont => enable = false,
        }
    }
    sum
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut sum = 0;
    for ist in input {
        if let Instruction::Mul(x, y) = ist {
            sum += x * y;
        }
    }
    sum
}

#[test]
fn test_1() {
    let gen = &generator("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    println!("{:?}", gen);
    assert_eq!(gen.len(), 4);
    let result = solve_part1(gen);
    assert_eq!(result, 161);
}

#[test]
fn test_2() {
    let gen = &generator("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    println!("{:?}", gen);
    let result = solve_part2(gen);
    assert_eq!(result, 48);
}
