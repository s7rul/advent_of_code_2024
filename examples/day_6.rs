use advent_of_code_2022::read_input_to_vec;

fn main() {
    println!("Day 6");
    let input = read_input_to_vec("input/day6.txt");
    let (times, records) = parse(&input);

    let mut product = 1;
    for i in 0..times.len() {
        product *= get_no_winning_strategies(times[i], records[i]);
    }

    println!("Part 1: {product}");

    let (time, record) = parse2(&input);

    println!("time: {time}");
    println!("record: {record}");

    let result = get_no_winning_strategies(time, record);
    println!("Part 2: {result}");
}

fn parse(input: &Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let times = input[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let records = input[1]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    (times, records)
}

fn parse2(input: &Vec<String>) -> (u64, u64) {
    let time = input[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let record = input[1]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    (time, record)
}

fn get_no_winning_strategies(time: u64, record: u64) -> u64 {
    let mut count = 0;
    for i in 1..time {
        if (i * (time - i)) > record {
            count += 1;
        }
    }
    count
}
