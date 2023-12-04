use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn first(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut sum: u32 = 0;
    for line in lines {
        let line = line?;

        let (_, line) = line.split_once(':').unwrap();
        let (winning, drawn) = line.split_once('|').unwrap();
        let winning: Vec<u32> = winning
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let count = drawn
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning.contains(x))
            .count() as u32;
        if count > 0 {
            sum += 2_u32.pow(count - 1);
        }
    }
    Ok(sum)
}

fn second(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut card_wins = vec![];

    for line in lines {
        let line = line?;

        let (_, line) = line.split_once(':').unwrap();
        let (winning, drawn) = line.split_once('|').unwrap();
        let winning: Vec<u32> = winning
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let count = drawn
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning.contains(x))
            .count();
        card_wins.push(count);
    }
    println!("{:?}", card_wins);

    let mut cards = vec![1; card_wins.len()];

    for i in 0..cards.len() {
        let card_cnt = cards[i];
        for j in 0..card_wins[i] {
            if i + j + 1 >= cards.len() {
                break;
            }
            cards[i + j + 1] += card_cnt;
        }
    }
    println!("{:?}", cards);
    Ok(cards.iter().sum())
}

fn main() -> Result<()> {
    let file = File::open("input/4.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", first(reader)?);
    println!("{}", second(reader)?);

    Ok(())
}
