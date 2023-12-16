use anyhow::Result;
use std::fs;

fn part_one(file_contents: String) -> usize {
    1
}

fn part_two(file_contents: String) -> usize {
    1
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input/16.txt")?;

    println!("{}", part_one(file_contents));
    // println!("{}", part_two(grid));

    Ok(())
}
