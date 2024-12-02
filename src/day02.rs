use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|l| {
        let repport = l.split_whitespace().map(|n| {n.parse::<i32>().unwrap()}).collect();
        repport
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let mut safe = 0;
    'out: for repport in input {
        let mut increasing = false;
        let mut first_comp = true;
        let mut last = repport[0];
        for n in &repport[1..] {
            let diff = last - n;
            if diff.abs() > 3 || diff.abs() == 0 {
                continue 'out;
            }

            if first_comp {
                increasing = diff < 0;
                first_comp = false;
            } else if increasing {
                if diff > 0 {
                    continue 'out;
                }
            } else if diff < 0 {
                continue 'out;
            }

            last = *n;
        }
        safe += 1;
    }
    safe
}

fn is_safe(repport: &[i32]) -> bool {
    let mut increasing = false;
    let mut first_comp = true;
    let mut last = repport[0];
    for n in &repport[1..] {
        let diff = last - n;
        if diff.abs() > 3 || diff.abs() == 0 {
            return false;
        }

        if first_comp {
            increasing = diff < 0;
            first_comp = false;
        } else if increasing {
            if diff > 0 {
                return false;
            }
        } else if diff < 0 {
            return false;
        }

        last = *n;
    }
    true
}

#[aoc(day2, part2, test_2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    let mut safe = 0;
    for repport in input {
        if is_safe(repport) {
            safe += 1;
        } else {
            for i in 0..repport.len() {
                let mut repport_copy = repport.clone();
                repport_copy.remove(i);
                if is_safe(&repport_copy) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

//#[aoc(day2, part2)]
//pub fn solve_part2(input: &str) -> i32 {
//    0
//}

#[test]
fn test_part_1() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve_part1(&generator(&input));
    assert!(solution == 2);
}

#[test]
fn test_part_2() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve_part2(&generator(&input));
    assert!(solution == 4);
}

#[test]
fn test_part_2_2() {
    let input = "7 15 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve_part2(&generator(&input));
    assert!(solution == 3);
}

#[test]
fn test_part_2_3() {
    let input = "7 8 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve_part2(&generator(&input));
    assert!(solution == 4);
}

#[test]
fn test_part_2_4() {
    let input = "1 0 3 4 5
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve_part2(&generator(&input));
    assert!(solution == 4);
}
