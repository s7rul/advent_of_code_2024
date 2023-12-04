use advent_of_code_2022::read_input_to_vec;

type EngineMap = Vec<Vec<char>>;

fn parse(input: &Vec<String>) -> EngineMap {
    let mut ret = vec![];
    for line in input {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        ret.push(row);
    }
    ret
}

fn check_for_adjacent_symbol(engine_map: &EngineMap, x: usize, y: usize) -> bool {
    // left
    if x > 0 {
        let c = engine_map[y][x - 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // right
    if x < engine_map[0].len() - 1 {
        let c = engine_map[y][x + 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // up
    if y > 0 {
        let c = engine_map[y - 1][x];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // down
    if y < engine_map.len() - 1 {
        let c = engine_map[y + 1][x];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // up left
    if y > 0 && x > 0 {
        let c = engine_map[y - 1][x - 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // up right
    if y > 0 && x < engine_map[0].len() - 1 {
        let c = engine_map[y - 1][x + 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // down left
    if y < engine_map.len() - 1 && x > 0 {
        let c = engine_map[y + 1][x - 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }
    // down right
    if y < engine_map.len() - 1 && x < engine_map[0].len() - 1 {
        let c = engine_map[y + 1][x + 1];
        if c != '.' && !c.is_digit(10) {
            return true;
        }
    }

    false
}

fn find_part_numbers(engine_map: &EngineMap) -> Vec<u32> {
    let mut ret = vec![];
    let mut in_part_number = false;
    let mut last_number = vec![];

    for y in 0..engine_map.len() {
        for x in 0..engine_map[0].len() {
            let c = engine_map[y][x];

            if c.is_digit(10) {
                last_number.push(c);

                if !in_part_number {
                    in_part_number = check_for_adjacent_symbol(engine_map, x, y);
                }
            }

            if !c.is_digit(10) || x == engine_map[0].len() - 1 {
                if in_part_number {
                    //println!("part number found: {:?}", last_number);
                    let number: String = last_number.iter().collect();
                    let number: u32 = number.parse().unwrap();
                    ret.push(number);
                }

                in_part_number = false;
                last_number.clear();
            }
        }
    }

    ret
}

#[derive(Debug)]
struct PartNumber {
    x_start: usize,
    x_end: usize,
    y: usize,
    number: u32,
}

fn find_all_numbers(engine_map: &EngineMap) -> Vec<PartNumber> {
    let mut last_number = vec![];
    let mut ret = vec![];
    for y in 0..engine_map.len() {
        for x in 0..engine_map[0].len() {
            let c = engine_map[y][x];

            if c.is_digit(10) {
                last_number.push(c);
            }

            if (!c.is_digit(10) || x == engine_map[0].len() - 1) && !last_number.is_empty() {
                let number: String = last_number.iter().collect();
                let number: u32 = number.parse().unwrap();
                let (x_start, x_end) = if c.is_digit(10) {
                    (x - (last_number.len() - 1), x)
                } else {
                    (x - last_number.len(), x - 1)
                };
                let part_nr = PartNumber {
                    x_start,
                    x_end,
                    y,
                    number,
                };

                ret.push(part_nr);

                last_number.clear();
            }
        }
    }
    ret
}

fn is_adjacent(part_number: &PartNumber, x: usize, y: usize) -> bool {
    // left
    if x > 0 {
        if part_number.x_end == x - 1 && part_number.y == y {
            return true;
        }
    }
    // right
    if part_number.x_start == x + 1 && part_number.y == y {
        return true;
    }
    // up
    if y > 0 {
        if part_number.x_start <= x && part_number.x_end >= x && part_number.y == y - 1 {
            return true;
        }
    }
    // down
    if part_number.x_start <= x && part_number.x_end >= x && part_number.y == y + 1 {
        return true;
    }
    // up left
    if y > 0 && x > 0 {
        if part_number.x_end == x - 1 && part_number.y == y - 1 {
            return true;
        }
    }
    // up right
    if y > 0 {
        if part_number.x_start == x + 1 && part_number.y == y - 1 {
            return true;
        }
    }
    // down left
    if x > 0 {
        if part_number.x_end == x - 1 && part_number.y == y + 1 {
            return true;
        }
    }
    // down right
    if part_number.x_start == x + 1 && part_number.y == y + 1 {
        return true;
    }

    false
}

fn find_adjacent_numbers(part_numbers: &Vec<PartNumber>, x: usize, y: usize) -> Vec<u32> {
    let mut ret = vec![];
    for n in part_numbers {
        if is_adjacent(n, x, y) {
            ret.push(n.number);
        }
    }
    ret
}

fn find_gear_ratios(engine_map: &EngineMap) -> Vec<u32> {
    let part_numbers = find_all_numbers(engine_map);

    let mut ret = vec![];
    for y in 0..engine_map.len() {
        for x in 0..engine_map[0].len() {
            if engine_map[y][x] == '*' {
                let numbers = find_adjacent_numbers(&part_numbers, x, y);
                if numbers.len() == 2 {
                    ret.push(numbers[0] * numbers[1]);
                }
            }
        }
    }
    ret
}

fn main() {
    println!("Day 3");
    let input = read_input_to_vec("input/day3.txt");

    let engine_map = parse(&input);
    let part_numbers = find_part_numbers(&engine_map);
    let mut sum = 0;
    for n in part_numbers {
        sum += n;
    }
    println!("part1: {}", sum);

    let gear_ratios = find_gear_ratios(&engine_map);
    let sum: u32 = gear_ratios.iter().sum();
    println!("part2: {}", sum);
}
