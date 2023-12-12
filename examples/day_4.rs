use std::{collections::HashSet, time::Instant};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let start = Instant::now();
    println!("Day 4");
    let input = read_input_to_vec("input/day4.txt");

    let cards = parse(&input);
    let mut sum = 0;

    for card in &cards {
        sum += card.calculate_points();
    }

    println!("part 1: {}", sum);

    let mut owned_cards = vec![1; cards.len()];

    for card in &cards {
        let wins = card.get_wins();
        let multiplier = owned_cards[card.id - 1];

        if wins == 0 {
            continue;
        }

        for i in (card.id)..(card.id + wins as usize) {
            owned_cards[i] += multiplier;
        }
    }

    let sum: u32 = owned_cards.iter().sum();
    println!("part 2: {}", sum);
    println!("time: {:?}", start.elapsed());
}

impl Card {
    fn calculate_points(&self) -> u32 {
        let mut points = 0;

        for n in &self.numbers {
            if self.winning_number.contains(n) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        points
    }

    fn get_wins(&self) -> u32 {
        let mut wins = 0;

        for n in &self.numbers {
            if self.winning_number.contains(n) {
                wins += 1;
            }
        }

        wins
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: Vec<u32>,
    winning_number: HashSet<u32>,
}

fn parse_card(input: &str) -> Card {
    let (id, numbers) = input.split_once(':').unwrap();

    let id = id.split_whitespace().collect::<Vec<&str>>()[1];
    let id: usize = id.parse().unwrap();

    let (winning, numbers) = numbers.split_once('|').unwrap();

    let winning: HashSet<u32> = winning
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let numbers: Vec<u32> = numbers
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    Card {
        id,
        numbers,
        winning_number: winning,
    }
}

fn parse(input: &Vec<String>) -> Vec<Card> {
    let mut ret = vec![];
    for line in input {
        ret.push(parse_card(&line));
    }
    ret
}
