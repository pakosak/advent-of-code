use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part_one(reader: BufReader<File>) -> u32 {
    let lines = reader.lines();
    1
}

fn part_two(reader: BufReader<File>) -> u32 {
    1
}

fn main() -> Result<()> {
    let file = File::open("input/1.txt")?;
    let reader = io::BufReader::new(file);

    println!("{}", part_one(reader));
    // println!("{}", part_two(reader));

    Ok(())
}
