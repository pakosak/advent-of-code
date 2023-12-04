use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn first(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();
    Ok(1)
}

fn second(reader: BufReader<File>) -> Result<u32> {
    Ok(1)
}

fn main() -> Result<()> {
    let file = File::open("input/1.txt")?;
    let reader = io::BufReader::new(file);

    println!("{}", first(reader)?);
    // println!("{}", second(reader)?);

    Ok(())
}
