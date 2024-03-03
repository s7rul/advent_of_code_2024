use std::time::Instant;

use advent_of_code_2022::read_input_file;

fn main() {
    let time = Instant::now();
    println!("Day 15");
    let input = read_input_file("input/day15.txt");

    let mut sum: u128 = 0;
    let mut map = AoCHashMap::new();

    for string in input.split(',') {
        sum += hash(string) as u128;

        let instruction: Instruction = string.try_into().unwrap();

        match instruction.operation {
            Operation::Add(v) => map.insert(Lens { label: instruction.label, focal_length: v }),
            Operation::Remove => map.remove_label(&instruction.label),
        }
    }
    println!("Part 1: {sum}");

    let result = map.calculate_focusing_power();
    println!("Part 2: {result}");

    println!("Time: {:?}", time.elapsed());
}

fn hash(input: &str) -> u8 {
    let mut sum: u8 = 0;

    for c in input.bytes() {
        (sum, _) = sum.overflowing_add(c);
        (sum, _) = sum.overflowing_mul(17);
    }

    sum
}

enum Operation {
    Add(u64),
    Remove,
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut label = vec![];
        let mut operation = None;
        for (i, c) in value.chars().enumerate() {
            if c == '=' {
                let focal_length: u64 = value[(i+1)..].parse().unwrap();
                operation = Some(Operation::Add(focal_length));
                break;
            } else if c == '-' {
                operation = Some(Operation::Remove);
                break;
            } else {
                label.push(c);
            }
        }
        let label: String = label.iter().collect();
        Ok(Instruction { label, operation: operation.unwrap() })
    }
}

struct Instruction {
    label: String,
    operation: Operation,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u64,
}

struct AoCHashMap {
    boxes: [Vec<Lens>; 256],
}

impl AoCHashMap {
    fn new() -> Self {

        let mut boxes: Vec<Vec<Lens>> = vec![];
        for i in 0..256 {
            boxes.push(vec![]);
        }
        AoCHashMap { boxes: boxes.try_into().unwrap() }
    }

    fn insert(&mut self, item: Lens) {
        let hash = hash(&item.label);
        let lenses = &mut self.boxes[hash as usize];

        for i in 0..lenses.len() {
            if lenses[i].label == item.label {
                lenses[i] = item;
                return;
            }
        }

        lenses.push(item);
    }

    fn remove_label(&mut self, label: &str) {
        let hash = hash(label);
        let lenses = &mut self.boxes[hash as usize];

        for i in 0..lenses.len() {
            if lenses[i].label == label {
                let _ = lenses.remove(i);
                return;
            }
        }
    }

    fn calculate_focusing_power(&self) -> u64 {
        let mut sum = 0;

        for (bi, b) in self.boxes.iter().enumerate() {
            for (li, l) in b.iter().enumerate() {
                sum += (bi + 1) as u64 * (li + 1) as u64 * l.focal_length;
            }
        }

        sum
    }
}
