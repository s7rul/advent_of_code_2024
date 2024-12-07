use std::fs;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Calibration {
    result: u64,
    numbers: Vec<u64>,
}

impl Calibration {
    fn parse(input: &str) -> Calibration {
        let (result, nums) = input.split_once(':').unwrap();
        let result: u64 = result.parse().unwrap();
        let numbers: Vec<u64> = nums.split_whitespace().map(|v| v.parse().unwrap()).collect();
        Calibration { result, numbers }
    }

    fn is_valid(&self) -> bool {
        for n in 0..(2_u64.pow(self.numbers.len() as u32 - 1)) {
            let mut result = self.numbers[0];
            for i in 1..self.numbers.len() {
                if (n & (1<<(i - 1))) > 0 {
                    result *= self.numbers[i];
                } else {
                    result += self.numbers[i];
                }
            }
            if result == self.result {
                return true;
            }
        }
        false
    }

    fn is_valid_optimized(&self) -> bool {
        'out: for n in 0..(2_u64.pow(self.numbers.len() as u32 - 1)) {
            let mut result = self.numbers[0];
            for i in 1..self.numbers.len() {
                if (n & (1<<(i - 1))) > 0 {
                    result *= self.numbers[i];
                } else {
                    result += self.numbers[i];
                }
                if result > self.result {
                    continue 'out;
                }
            }
            if result == self.result {
                return true;
            }
        }
        false
    }

    fn is_valid_2(&self) -> bool {
        for mut n in 0..(3_u64.pow(self.numbers.len() as u32 - 1)) {
            let mut result = self.numbers[0];
            for i in 1..self.numbers.len() {
                result = match n % 3 {
                    0 => result + self.numbers[i],
                    1 => result * self.numbers[i],
                    2 => {
                        let mut inter = result.to_string();
                        inter.push_str(&self.numbers[i].to_string());
                        inter.parse().unwrap()
                    }
                    _ => panic!("should not get here")
                };
                n /= 3;
            }
            if result == self.result {
                return true;
            }
        }
        false
    }

    fn is_valid_2_optimized(&self) -> bool {
        'out: for mut n in 0..(3_u64.pow(self.numbers.len() as u32 - 1)) {
            let mut result = self.numbers[0];
            for i in 1..self.numbers.len() {
                result = match n % 3 {
                    0 => result + self.numbers[i],
                    1 => result * self.numbers[i],
                    2 => {
                        let mut inter = result.to_string();
                        inter.push_str(&self.numbers[i].to_string());
                        inter.parse().unwrap()
                    }
                    _ => panic!("should not get here")
                };
                if result > self.result {
                    continue 'out;
                }
                n /= 3;
            }
            if result == self.result {
                return true;
            }
        }
        false
    }
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<Calibration> {
    input.lines().map(Calibration::parse).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Calibration]) -> u64 {
    input.iter().filter(|c| c.is_valid()).map(|c| c.result).sum()
}

#[aoc(day7, part1, optimized)]
pub fn solve_part1_optimized(input: &[Calibration]) -> u64 {
    input.iter().filter(|c| c.is_valid_optimized()).map(|c| c.result).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Calibration]) -> u64 {
    input.iter().filter(|c| c.is_valid_2()).map(|c| c.result).sum()
}

#[aoc(day7, part2, optimized)]
pub fn solve_part2_optimized(input: &[Calibration]) -> u64 {
    input.iter().filter(|c| c.is_valid_2_optimized()).map(|c| c.result).sum()
}

#[test]
fn test_1() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let input = generator(input);
    let result = solve_part1(&input);
    assert_eq!(result, 3749);
}

#[test]
fn test_2() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let input = generator(input);
    let result = solve_part2(&input);
    assert_eq!(result, 11387);
}

#[test]
fn full_test_1_1() {
    let input = fs::read_to_string("input/2024/day7.txt").unwrap();
    let input = generator(&input);
    let result = solve_part1(&input);
    assert_eq!(result, 303876485655);
}

#[test]
fn full_test_1_2() {
    let input = fs::read_to_string("input/2024/day7.txt").unwrap();
    let input = generator(&input);
    let result = solve_part1_optimized(&input);
    assert_eq!(result, 303876485655);
}

#[test]
fn full_test_2_1() {
    let input = fs::read_to_string("input/2024/day7.txt").unwrap();
    let input = generator(&input);
    let result = solve_part2(&input);
    assert_eq!(result, 146111650210682);
}
#[test]
fn full_test_2_2() {
    let input = fs::read_to_string("input/2024/day7.txt").unwrap();
    let input = generator(&input);
    let result = solve_part2_optimized(&input);
    assert_eq!(result, 146111650210682);
}
