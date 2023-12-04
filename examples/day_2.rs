use advent_of_code_2022::read_input_to_vec;

fn validate_part1(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        if validate_game_part1(game) {
            sum += game.id;
        }
    }

    sum
}

fn validate_game_part1(game: &Game) -> bool {
    for reveal in &game.reveals {
        match reveal {
            Color::Red(v) => {
                if *v > 12 {
                    return false;
                }
            }
            Color::Green(v) => {
                if *v > 13 {
                    return false;
                }
            }
            Color::Blue(v) => {
                if *v > 14 {
                    return false;
                }
            }
        }
    }
    true
}

fn find_min_cubes(game: &Game) -> (u32, u32, u32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for reveal in &game.reveals {
        match reveal {
            Color::Red(v) => {
                min_red = min_red.max(*v);
            }
            Color::Green(v) => {
                min_green = min_green.max(*v);
            }
            Color::Blue(v) => {
                min_blue = min_blue.max(*v);
            }
        }
    }
    (min_red, min_green, min_blue)
}

fn calculate_min_powers_sum(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let (r, g, b) = find_min_cubes(game);
            r * g * b
        })
        .sum()
}

fn main() {
    println!("day 2");
    //let input = read_input_to_vec("input/day2_test.txt");
    let input = read_input_to_vec("input/day2.txt");

    let mut games = vec![];
    for line in input {
        games.push(parse(&line));
    }

    let part1 = validate_part1(&games);
    println!("part1: {}", part1);
    println!("part2: {}", calculate_min_powers_sum(&games));
}

fn parse(input: &str) -> Game {
    let split: Vec<&str> = input.split_terminator(&[':', ',', ';']).collect();
    let id: Vec<&str> = split[0].split_whitespace().collect();
    let id = id[1];
    let id = id.parse::<u32>().unwrap();

    let mut list = vec![];
    for c in &split[1..] {
        list.push(parse_color(c));
    }
    Game { id, reveals: list }
}

fn parse_color(input: &str) -> Color {
    let split: Vec<&str> = input.trim().split_whitespace().collect();
    match split[1] {
        "red" => Color::Red(split[0].parse().unwrap()),
        "green" => Color::Green(split[0].parse().unwrap()),
        "blue" => Color::Blue(split[0].parse().unwrap()),
        _ => panic!(),
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Color>,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}
