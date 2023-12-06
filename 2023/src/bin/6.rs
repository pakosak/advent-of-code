use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn race_dist(wait: u64, duration: u64) -> u64 {
    (duration - wait) * wait
}

fn first(reader: BufReader<File>) -> Result<u64> {
    let lines = reader.lines();
    let parsed = lines
        .map(|l| {
            l.unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let races = parsed[0].iter().zip(parsed[1].iter());
    println!("{:?}", races);

    Ok(races
        .map(|(time, old_record)| {
            (0..*time)
                .map(|t| race_dist(t, *time))
                .filter(|distance| distance > old_record)
                .count() as u64
        })
        .product())
}

fn second(reader: BufReader<File>) -> Result<u64> {
    let lines = reader.lines();
    let parsed = lines
        .map(|l| {
            l.unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .fold("".to_string(), |acc, x| acc + x)
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let time = parsed[0];
    let dist = parsed[1];
    println!("{:?} {}", time, dist);

    Ok((0..time)
        .map(|t| race_dist(t, time))
        .filter(|distance| distance > &dist)
        .count() as u64)
}

fn main() -> Result<()> {
    let file = File::open("input/6.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", first(reader)?);
    println!("{}", second(reader)?);

    Ok(())
}
