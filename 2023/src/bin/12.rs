use anyhow::Result;
use itertools::Itertools;
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
    springs
        .char_indices()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .combinations(remaining)
        .filter(|comb| is_valid_combination(comb, &springs, &counts))
        .count()
}

fn part_one(reader: BufReader<File>) -> usize {
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (springs, counts) = l.split_once(' ').unwrap();
            possible_combination_count(
                springs.to_string(),
                counts
                    .split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn part_two(reader: BufReader<File>) -> u32 {
    1
}

fn main() -> Result<()> {
    let file = File::open("input/12.txt")?;
    let reader = io::BufReader::new(file);

    println!("{}", part_one(reader));
    // println!("{}", part_two(reader));

    Ok(())
}
