use std::{collections::HashMap, fmt::Display, time::Instant};

use advent_of_code_2022::read_input_to_vec;

fn main() {
    let time = Instant::now();
    println!("Day 12");
    let input = read_input_to_vec("input/day12.txt");
    let spring_rows: Vec<SimpleRow> = input.iter().map(|s| s.try_into().unwrap()).collect();

    //let one = spring_rows[5].solve1();
    //todo!();

    let mut cache = HashMap::new();

    let mut sum = 0;
    for r in &spring_rows {
        let result = r.solve(&mut cache);
        //println!("result: {result}");
        sum += result;
    }
    println!("Part 1: {sum}");

    let mut sum = 0;
    for r in &spring_rows {
        let result = r.unfold().solve(&mut cache);
        //println!("result: {result}");
        sum += result;
    }
    println!("part 2: {sum}");

    println!("time: {:?}", time.elapsed());
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SimpleRow {
    springs: Vec<SpringCondition>,
    ds_numbers: Vec<u64>,
}

impl SimpleRow {
    fn unfold(&self) -> SimpleRow {
        let mut new_springs = vec![];
        let mut new_numbers = vec![];
        for i in 0..5 {
            for s in &self.springs {
                new_springs.push(s.clone());
            }

            if i != 4 {
                new_springs.push(SpringCondition::Unknown);
            }

            for n in &self.ds_numbers {
                new_numbers.push(*n)
            }
        }
        SimpleRow {
            springs: new_springs,
            ds_numbers: new_numbers,
        }
    }

    fn solve(&self, cache: &mut HashMap<Self, u128>) -> u128 {
        let mut unknown = vec![];
        for (i, s) in self.springs.iter().enumerate() {
            match s {
                SpringCondition::Unknown => unknown.push(i),
                _ => (),
            }
        }
        self.get_no_solutions(cache)
    }

    fn is_valid(&self) -> Option<bool> {
        //print!("{}", self);
        let mut count = 0;
        let mut i = 0;
        let mut last = SpringCondition::Operational;
        for s in &self.springs {
            match (&last, s) {
                (SpringCondition::Damaged, SpringCondition::Operational) => {
                    if i == self.ds_numbers.len() {
                        return Some(false);
                    }
                    if self.ds_numbers[i] != count {
                        //println!(" invalid");
                        return Some(false);
                    }
                    i += 1;
                    count = 0;
                }
                (_, SpringCondition::Damaged) => {
                    count += 1;
                }
                (_, SpringCondition::Unknown) => return None,
                (_, _) => (),
            }
            last = s.clone();
        }
        if count > 0 {
            if i < self.ds_numbers.len() {
                if self.ds_numbers[i] != count {
                    //println!(" invalid");
                    return Some(false);
                }
                i += 1;
            } else {
                return Some(false);
            }
        }

        if i == self.ds_numbers.len() {
            //print!("{}", self);
            //println!(" valid");
            Some(true)
        } else {
            //println!(" invalid");
            Some(false)
        }
    }

    // Almost copy of sopby:s solution
    fn get_no_solutions(&self, cache: &mut HashMap<Self, u128>) -> u128 {
        if self.springs.is_empty() {
            if self.ds_numbers.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        match self.springs[0] {
            SpringCondition::Damaged => self.get_no_damaged_solutions(cache),
            SpringCondition::Operational => (SimpleRow {
                springs: self.springs[1..].to_vec(),
                ds_numbers: self.ds_numbers.clone(),
            })
            .get_no_solutions(cache),
            SpringCondition::Unknown => {
                (SimpleRow {
                    springs: self.springs[1..].to_vec(),
                    ds_numbers: self.ds_numbers.clone(),
                })
                .get_no_solutions(cache)
                    + self.get_no_damaged_solutions(cache)
            }
        }
    }

    fn get_no_damaged_solutions(&self, cache: &mut HashMap<Self, u128>) -> u128 {
        if let Some(result) = cache.get(self) {
            return *result;
        }

        if self.ds_numbers.is_empty() {
            return 0;
        }

        let ds_number = self.ds_numbers[0];
        if self.springs.len() < ds_number as usize {
            return 0;
        }

        for i in 0..ds_number as usize {
            if self.springs[i] == SpringCondition::Operational {
                return 0;
            }
        }

        if self.springs.len() == ds_number as usize {
            if self.ds_numbers.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }
        if self.springs[ds_number as usize] == SpringCondition::Damaged {
            return 0;
        }

        let result = (SimpleRow {
            springs: self.springs[(ds_number + 1) as usize..].to_vec(),
            ds_numbers: self.ds_numbers[1..].to_vec(),
        })
        .get_no_solutions(cache);
        cache.insert(self.clone(), result);
        result
    }
}

impl Display for SimpleRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for s in &self.springs {
            write!(f, "{}", s)?
        }
        write!(f, "] (")?;
        for n in &self.ds_numbers {
            write!(f, "{}, ", n)?
        }
        write!(f, ")")
    }
}

impl TryFrom<&String> for SimpleRow {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (springs, ds_numbers) = value.split_once(' ').unwrap();

        let ds_numbers = ds_numbers.split(',').map(|v| v.parse().unwrap()).collect();
        Ok(SimpleRow {
            springs: springs.chars().map(|v| v.try_into().unwrap()).collect(),
            ds_numbers,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum SpringCondition {
    Damaged,
    Operational,
    Unknown,
}

impl Display for SpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringCondition::Damaged => write!(f, "#"),
            SpringCondition::Operational => write!(f, "."),
            SpringCondition::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for SpringCondition {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(SpringCondition::Operational),
            '#' => Ok(SpringCondition::Damaged),
            '?' => Ok(SpringCondition::Unknown),
            _ => Err("invalid spring"),
        }
    }
}

#[derive(Debug, Clone)]
struct DiscreteRange {
    groups: Vec<SpringGroup>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SpringGroup {
    condition: SpringCondition,
    length: u64,
}

#[derive(Debug)]
struct SpringRow {
    spring_groups: Vec<SpringGroup>,
    ds_numbers: Vec<u64>,
}

impl SpringRow {
    fn solve1(&self) -> u64 {
        let mut discreet_ranges = 0;
        let mut descrete_groups = vec![];
        let mut last_group = vec![];
        let mut last_group_condition = SpringCondition::Operational;

        for group in &self.spring_groups {
            if last_group_condition == SpringCondition::Operational
                && group.condition != SpringCondition::Operational
            {
                if !last_group.is_empty() {
                    descrete_groups.push(DiscreteRange {
                        groups: last_group.clone(),
                    });
                    last_group.clear();
                }
                discreet_ranges += 1;
            }
            last_group_condition = group.condition.clone();
            if group.condition != SpringCondition::Operational {
                last_group.push(group.clone());
            }
        }
        if !last_group.is_empty() {
            descrete_groups.push(DiscreteRange {
                groups: last_group.clone(),
            });
            last_group.clear();
        }
        println!("Descrete ranges: {discreet_ranges}");

        0
    }
}

impl TryFrom<&String> for SpringRow {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let (springs, ds_numbers) = value.split_once(' ').unwrap();

        let ds_numbers = ds_numbers.split(',').map(|v| v.parse().unwrap()).collect();

        let mut spring_groups = vec![];

        let mut last_spring: Option<SpringCondition> = None;
        let mut last_group_end = 0;
        for (i, c) in springs.chars().enumerate() {
            match &last_spring {
                Some(s) => {
                    let current = c.try_into()?;
                    if *s != current {
                        spring_groups.push(SpringGroup {
                            condition: s.clone(),
                            length: (i - last_group_end) as u64,
                        });
                        last_spring = Some(current.clone());
                        last_group_end = i;
                    }
                }
                None => last_spring = Some(c.try_into()?),
            }

            if i == springs.len() - 1 {
                spring_groups.push(SpringGroup {
                    condition: last_spring.clone().unwrap().clone(),
                    length: (i - last_group_end) as u64 + 1,
                });
            }
        }

        Ok(SpringRow {
            spring_groups,
            ds_numbers,
        })
    }
}
