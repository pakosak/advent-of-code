use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_valid_combination(comb: &[usize], springs: &str, counts: &[u32]) -> bool {
    let mut count_idx = 0;
    let mut current_cluster: Option<u32> = None;
    for (i, c) in springs.char_indices() {
        if c == '#' || comb.contains(&i) {
            current_cluster = Some(current_cluster.unwrap_or_default() + 1);
        } else {
            if let Some(cluster) = current_cluster {
                if cluster != counts[count_idx] {
                    return false;
                }
                count_idx += 1;
            }
            current_cluster = None;
        }
    }
    if let Some(cluster) = current_cluster {
        if cluster != counts[count_idx] {
            return false;
        }
    }
    true
}

fn possible_combination_count(springs: String, counts: Vec<u32>) -> usize {
    let remaining: usize =
        counts.iter().sum::<u32>() as usize - springs.chars().filter(|c| *c == '#').count();
    // let possible_places = springs.chars().filter(|c| *c == '?').count();
    // let possible_combinations = springs
    //     .char_indices()
    //     .filter(|(_, c)| *c == '?')
    //     .map(|(i, _)| i)
    //     .combinations(remaining)
    //     .count();

    let count = springs
        .char_indices()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .combinations(remaining)
        .filter(|comb| is_valid_combination(comb, &springs, &counts))
        .count();
    println!("{count}");
    // println!(
    //     "{}\t{:?}\trem:{}\tpos:{}\tpos_res:{}\tres:{}",
    //     springs, counts, remaining, possible_places, possible_combinations, count
    // );
    count
}

fn solve(reader: BufReader<File>, multiplier: u32) -> usize {
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (springs, counts) = l.split_once(' ').unwrap();
            let mut springs = (0..multiplier).fold(String::new(), |acc, _| acc + springs + "?");
            springs.pop();
            let mut counts = (0..multiplier).fold(String::new(), |acc, _| acc + counts + ",");
            counts.pop();
            let counts = counts
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            possible_combination_count(springs, counts)
        })
        .sum()
}

fn part_one(reader: BufReader<File>) -> usize {
    solve(reader, 1)
}

fn part_two(reader: BufReader<File>) -> usize {
    solve(reader, 1)
}

fn main() -> Result<()> {
    let file = File::open("input/12.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", part_one(reader));
    println!("{}", part_two(reader));

    Ok(())
}
