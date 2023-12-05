use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Range {
    from: u64,
    to: u64,
}

type Mapping = HashMap<Range, i64>;

fn apply_mapping(mapping: &Mapping, seed: u64) -> u64 {
    for (range, offset) in mapping {
        if range.from <= seed && seed < range.to {
            // println!("{} -> {}", seed, seed as i64 + offset);
            return (seed as i64 + offset) as u64;
        }
    }
    // println!("{} -> {}", seed, seed);
    seed
}

fn parse(reader: BufReader<File>) -> Result<(Vec<u64>, Vec<Mapping>)> {
    let mut lines = reader.lines();

    let seeds = lines
        .next()
        .unwrap()?
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps: Vec<Mapping> = Vec::new();
    lines.next();
    let mut current_map: Mapping = HashMap::new();
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            // println!("{:?}", current_map);
            maps.push(current_map);
            current_map = HashMap::new();
            continue;
        }
        if line.contains("map") {
            continue;
        }
        let nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let range = Range {
            from: nums[1],
            to: nums[1] + nums[2],
        };
        current_map.insert(range, nums[0] as i64 - nums[1] as i64);
    }

    Ok((seeds, maps))
}

fn first(seeds: Vec<u64>, maps: Vec<Mapping>) -> Result<u64> {
    Ok(seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |acc, m| apply_mapping(m, acc)))
        .min()
        .unwrap())
}

fn second(seeds: Vec<u64>, maps: Vec<Mapping>) -> Result<u64> {
    Ok(seeds
        .windows(2)
        .step_by(2)
        .map(|window| {
            println!("{} {}", window[0], window[1]);
            (window[0]..(window[0] + window[1]))
                .into_par_iter()
                .map(|s| maps.iter().fold(s, |acc, m| apply_mapping(m, acc)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap())
}

fn main() -> Result<()> {
    let file = File::open("input/5.txt")?;
    let reader = io::BufReader::new(file);

    let (seeds, maps) = parse(reader)?;
    // println!("{}", first(seeds, maps)?);
    println!("{}", second(seeds, maps)?);

    Ok(())
}
