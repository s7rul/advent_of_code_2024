use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Rule {
    first: u32,
    secound: u32,
}

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    rules: HashMap<u32, Vec<u32>>,
    pages: Vec<Vec<u32>>,
}

#[aoc_generator(day5, part1)]
pub fn generator(input: &str) -> PuzzleInput {
    let mut in_pages = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages = vec![];
    for l in input.lines() {
        if l.is_empty() {
            in_pages = true;
            continue;
        }

        if in_pages {
            pages.push(l.split(',').map(|v| v.parse().unwrap()).collect());
        } else {
            let in_l: Vec<u32> = l.split('|').map(|v| v.parse().unwrap()).collect();

            let r = rules.entry(in_l[0]).or_insert(vec![]);
            r.push(in_l[1]);
        }
    }
    PuzzleInput { rules, pages }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &PuzzleInput) -> u32 {
    let mut valid_prints = vec![];

    for p in &input.pages {
        let mut must_contain = vec![];
        for n in p {
            let mut del_i = vec![];
            for i in 0..must_contain.len() {
                if must_contain[i] == *n {
                    del_i.push(i);
                }
            }
            for (offset, i) in del_i.into_iter().enumerate() {
                must_contain.remove(i - offset);
            }

            match input.rules.get(n) {
                Some(v) => for x in v {
                    if p.iter().find(|v| **v==*x).is_some() {
                        must_contain.push(*x);
                    }
                },
                None => (),
            }
        }

        if must_contain.is_empty() {
            valid_prints.push(p);
        }
    }

    let mut sum = 0;
    for vlid in valid_prints {
        let middle = vlid[vlid.len()/2];
        sum += middle;
    }

    sum
}

fn is_valid(pages: &[u32], rules: &HashMap<u32, Vec<(u32, Rule)>>) -> (bool, Vec<Rule>) {
    let mut must_contain: Vec<u32> = vec![];
    let mut affected_rules: Vec<Rule> = vec![];
    for n in pages {
        let mut del_i = vec![];
        for i in 0..must_contain.len() {
            if must_contain[i] == *n {
                del_i.push(i);
            }
        }
        for (offset, i) in del_i.into_iter().enumerate() {
            must_contain.remove(i - offset);
        }

        match rules.get(n) {
            Some(v) => for x in v {
                if pages.iter().find(|v| **v==x.0).is_some() {
                    must_contain.push(x.0);
                    affected_rules.push(x.1.clone());
                }
            },
            None => (),
        }
    }
    (must_contain.is_empty(), affected_rules)
}

#[aoc_generator(day5, part2)]
pub fn generator2(input: &str) -> PuzzleInput2 {
    let mut in_pages = false;
    let mut rules: HashMap<u32, Vec<(u32, Rule)>> = HashMap::new();
    let mut pages = vec![];
    for l in input.lines() {
        if l.is_empty() {
            in_pages = true;
            continue;
        }

        if in_pages {
            pages.push(l.split(',').map(|v| v.parse().unwrap()).collect());
        } else {
            let in_l: Vec<u32> = l.split('|').map(|v| v.parse().unwrap()).collect();

            let r = rules.entry(in_l[0]).or_insert(vec![]);
            r.push((in_l[1], Rule { first: in_l[0], secound: in_l[1] }));
        }
    }
    PuzzleInput2 { rules, pages }
}
#[derive(Debug, Clone)]
pub struct PuzzleInput2 {
    rules: HashMap<u32, Vec<(u32, Rule)>>,
    pages: Vec<Vec<u32>>,
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &PuzzleInput2) -> u32 {
    let mut fixed = vec![];
    for p in &input.pages {
        let (valid, affected_rules) = is_valid(p, &input.rules);
        if !valid {
        }
    }

    let mut sum = 0;
    for vlid in fixed {
        let middle = vlid[vlid.len()/2];
        sum += middle;
    }

    sum
}

#[test]
pub fn test2() {
    let input = generator2("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
    let result = solve_part2(&input);
    assert_eq!(result, 123);
}

#[test]
pub fn test1() {
    let input = generator("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
    let result = solve_part1(&input);
    assert_eq!(result, 143);
}
