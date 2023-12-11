use advent_of_code_2022::read_input_to_vec;

fn main() {
    println!("Day 7");

    let input = read_input_to_vec("input/day7.txt");
    let mut hands = parse(&input);
    hands.sort();
    for h in &hands {
        //println!("{:?}", h);
    }
    let result: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let ret = (i + 1) as u64 * c.bet;
            //println!("{},{ret}", i + 1);
            ret
        })
        .sum();

    println!("Part 1: {}", result);
}

fn parse(input: &Vec<String>) -> Vec<Hand> {
    let mut ret = vec![];
    for line in input {
        let (cards_str, bet) = line.split_once(' ').unwrap();
        let mut cards = [Card::C2; 5];
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = c.try_into().unwrap();
        }
        let bet = bet.trim().parse().unwrap();
        let hand_type = (&cards).try_into().unwrap();
        ret.push(Hand {
            bet,
            cards,
            hand_type,
        })
    }
    ret
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    High,
    OnePair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

impl TryFrom<&[Card; 5]> for HandType {
    type Error = &'static str;

    fn try_from(value: &[Card; 5]) -> Result<Self, Self::Error> {
        let mut buckets: [u8; 13] = [0; 13];
        for card in value {
            buckets[Into::<usize>::into(*card)] += 1;
        }
        let mut max = 0;
        let mut nex_max = 0;
        for bucket in &buckets[1..] {
            if *bucket > max {
                nex_max = max;
                max = *bucket;
            } else if *bucket > nex_max {
                nex_max = *bucket;
            }
        }
        if max < 5 {
            if max + buckets[0] <= 5 {
                max += buckets[0]
            }
        }
        //println!("cards: {:?}, max: {max}, nex_max: {nex_max}", value);
        match (max, nex_max) {
            (5, _) => Ok(HandType::Five),
            (4, _) => Ok(HandType::Four),
            (3, 2) => Ok(HandType::FullHouse),
            (3, _) => Ok(HandType::Three),
            (2, 2) => Ok(HandType::TwoPairs),
            (2, _) => Ok(HandType::OnePair),
            (1, _) => Ok(HandType::High),
            (_, _) => Err("not a handtype"),
        }
    }
}

#[derive(Debug, Ord, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bet: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cards.partial_cmp(&other.cards) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        panic!()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl Into<usize> for Card {
    fn into(self) -> usize {
        match self {
            Card::CJ => 0,
            Card::C2 => 1,
            Card::C3 => 2,
            Card::C4 => 3,
            Card::C5 => 4,
            Card::C6 => 5,
            Card::C7 => 6,
            Card::C8 => 7,
            Card::C9 => 8,
            Card::CT => 9,
            Card::CQ => 10,
            Card::CK => 11,
            Card::CA => 12,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::C2),
            '3' => Ok(Card::C3),
            '4' => Ok(Card::C4),
            '5' => Ok(Card::C5),
            '6' => Ok(Card::C6),
            '7' => Ok(Card::C7),
            '8' => Ok(Card::C8),
            '9' => Ok(Card::C9),
            'T' => Ok(Card::CT),
            'J' => Ok(Card::CJ),
            'Q' => Ok(Card::CQ),
            'K' => Ok(Card::CK),
            'A' => Ok(Card::CA),
            _ => Err("Not a card."),
        }
    }
}

#[test]
fn test_card_ordering() {
    assert!(Card::C2 < Card::CA);
    assert!(Card::C2 == Card::C2);
    assert!(Card::C8 > Card::C6);
}

#[test]
fn test_hand_ordering() {
    let hand1 = [Card::C2, Card::C2, Card::C2, Card::C2, Card::C2];
    let hand2 = [Card::C3, Card::C3, Card::C2, Card::C2, Card::C2];
    assert!(
        Hand {
            hand_type: HandType::Five,
            cards: hand1,
            bet: 0
        } > Hand {
            hand_type: HandType::FullHouse,
            cards: hand2,
            bet: 0
        }
    )
}
