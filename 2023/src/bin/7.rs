use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(hand: &Hand) -> HandType {
        let joker_count = hand.iter().filter(|&card| *card == Card::Joker).count();
        let mut counts: Vec<u32> = hand
            .iter()
            .filter(|&card| *card != Card::Joker)
            .fold(HashMap::new(), |mut counts, card| {
                *counts.entry(card).or_insert(0) += 1;
                counts
            })
            .into_values()
            .collect();
        counts.sort_by(|a, b| b.cmp(a));

        let unique: HashSet<Card> = HashSet::from_iter(hand.iter().cloned());
        match (unique.len(), counts.first().unwrap_or(&0), joker_count) {
            (1, 0, 5) => HandType::FiveOfAKind,
            (1, 5, 0) => HandType::FiveOfAKind,
            (2, 4, 1) => HandType::FiveOfAKind,
            (2, 3, 2) => HandType::FiveOfAKind,
            (2, 2, 3) => HandType::FiveOfAKind,
            (2, 1, 4) => HandType::FiveOfAKind,
            (2, 4, 0) => HandType::FourOfAKind,
            (3, 3, 1) => HandType::FourOfAKind,
            (3, 2, 2) => HandType::FourOfAKind,
            (3, 1, 3) => HandType::FourOfAKind,
            (2, 3, 0) => HandType::FullHouse,
            (3, 2, 1) => HandType::FullHouse,
            (3, 3, 0) => HandType::ThreeOfAKind,
            (4, 2, 1) => HandType::ThreeOfAKind,
            (4, 1, 2) => HandType::ThreeOfAKind,
            (3, 2, 0) => HandType::TwoPairs,
            (4, 2, 0) => HandType::OnePair,
            (5, 1, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(card: char, j_card: Card) -> Card {
        match card {
            'J' => j_card,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", card),
        }
    }
}

type Strength = (HandType, Hand);
type Hand = Vec<Card>;

fn strength(hand: Hand) -> Strength {
    (HandType::new(&hand), hand)
}

fn parse_hand(hand: &str, j_card: Card) -> Hand {
    hand.chars().map(|c| Card::new(c, j_card)).collect()
}

fn solve(reader: BufReader<File>, j_card: Card) -> Result<u32> {
    let lines = reader.lines();
    let mut plays: Vec<(Strength, u32)> = lines
        .map(|line| {
            let line = line.unwrap();
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let hand = parse_hand(hand, j_card);
            (strength(hand), bid)
        })
        .collect();
    plays.sort_by(|(l_hand, _), (r_hand, _)| l_hand.cmp(r_hand));
    println!("{:?}", plays);
    Ok(plays
        .iter()
        .enumerate()
        .map(|(i, play)| (i + 1) as u32 * play.1)
        .sum::<u32>())
}

fn first(reader: BufReader<File>) -> Result<u32> {
    solve(reader, Card::Jack)
}

fn second(reader: BufReader<File>) -> Result<u32> {
    solve(reader, Card::Joker)
}

fn main() -> Result<()> {
    let file = File::open("input/7.txt")?;
    let reader = io::BufReader::new(file);

    // println!("first: {}", first(reader)?);
    println!("second: {}", second(reader)?);

    Ok(())
}
