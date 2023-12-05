use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn first(reader: BufReader<File>) -> Result<u32> {
    let mut lines = reader.lines();

    let seeds = lines
        .next()
        .unwrap()?
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut maps: Vec<HashMap<u32, u32>>;
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if line.contains("map") {}
    }
    println!("{:?}", seeds);
    Ok(1)
}

fn second(reader: BufReader<File>) -> Result<u32> {
    Ok(1)
}

fn main() -> Result<()> {
    let file = File::open("input/5.txt")?;
    let reader = io::BufReader::new(file);

    println!("{}", first(reader)?);
    // println!("{}", second(reader)?);

    Ok(())
}
