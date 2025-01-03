use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Towel = Vec<char>;
type Pattern = Vec<char>;

pub struct Input {
    towels: Vec<Towel>,
    patterns: Vec<Pattern>,
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Input {
    let (towels_str, patterns_str) = input.split_once("\n\n").unwrap();
    let towels = towels_str.split(", ").map(|t| {
        t.chars().collect()
    }).collect();
    let patterns = patterns_str.lines().map(|l| l.chars().collect()).collect();

    Input { towels, patterns }
}

fn validate_pattern(towels: &[Towel], pattern: &Pattern, i: usize, cache: &mut HashMap<String, bool>) -> bool {
    if i >= pattern.len() {
        return true;
    }

    let pattern_string: String = pattern[i..].iter().collect();
    if let Some(r) = cache.get(&pattern_string) {
        return *r;
    }

    for t in towels {
        let t_len = t.len();

        if i + t_len > pattern.len() {
            continue;
        }

        if pattern[i..i + t_len] == *t {
            let result = validate_pattern(towels, pattern, i + t_len, cache);
            cache.insert(pattern[i + t_len..].iter().collect(), result);
            if result {
                return true;
            }
        }
    }
    false
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> i32 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for pattern in &input.patterns {
        if validate_pattern(&input.towels, pattern, 0, &mut cache) {
            sum += 1;
        }
    }
    sum
}

fn find_all(towels: &[Towel], pattern: &Pattern, i: usize, cache: &mut HashMap<String, u64>) -> u64 {
    if i >= pattern.len() {
        return 1;
    }

    let pattern_string: String = pattern[i..].iter().collect();
    if let Some(r) = cache.get(&pattern_string) {
        return *r;
    }

    let mut sum = 0;

    for t in towels {
        let t_len = t.len();

        if i + t_len > pattern.len() {
            continue;
        }

        if pattern[i..i + t_len] == *t {
            let result = find_all(towels, pattern, i + t_len, cache);
            cache.insert(pattern[i + t_len..].iter().collect(), result);
            sum += result;
        }
    }
    sum
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for pattern in &input.patterns {
        sum +=  find_all(&input.towels, pattern, 0, &mut cache);
    }
    sum
}

#[test]
fn test_2() {
    let input = generator("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb");
    let result = solve_part2(&input);
    assert_eq!(16, result);
}

#[test]
fn test_1() {
    let input = generator("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb");
    let result = solve_part1(&input);
    assert_eq!(6, result);
}
