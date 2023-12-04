use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn first(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut sum = 0;
    for line in lines {
        let line = line?;
        let first = line.clone().chars().find(|c| c.is_ascii_digit()).unwrap();
        let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        let num = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
        sum += num;
    }

    Ok(sum)
}

fn second(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    const NUMS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;
    for line in lines {
        let line = line?;

        let choose_num =
            |num_idx, word_idx: Option<(u32, usize)>, cmp: &dyn Fn(usize, usize) -> bool| {
                let get_nth_digit = |n: usize| line.chars().nth(n).unwrap().to_digit(10).unwrap();
                match (num_idx, word_idx) {
                    (Some(num_idx_uw), Some((word_num, word_idx_uw))) => {
                        if cmp(num_idx_uw, word_idx_uw) {
                            get_nth_digit(num_idx_uw)
                        } else {
                            word_num
                        }
                    }
                    (Some(num_idx_uw), None) => get_nth_digit(num_idx_uw),
                    (None, Some((word_num, _))) => word_num,
                    (None, None) => panic!("No number found"),
                }
            };

        let first_num_idx = line.find(|c: char| c.is_ascii_digit());
        let first_word_idx = NUMS
            .iter()
            .enumerate()
            .filter_map(|(i, word)| {
                if let Some(idx) = line.find(word) {
                    return Some(((i + 1) as u32, idx));
                }
                None
            })
            .min_by(|(_, lidx), (_, ridx)| lidx.cmp(ridx));

        let first = choose_num(first_num_idx, first_word_idx, &|l, r| l < r);

        let last_num_idx = line.rfind(|c: char| c.is_ascii_digit());
        let last_word_idx = NUMS
            .iter()
            .enumerate()
            .filter_map(|(i, word)| {
                if let Some(idx) = line.rfind(word) {
                    return Some(((i + 1) as u32, idx));
                }
                None
            })
            .max_by(|(_, lidx), (_, ridx)| lidx.cmp(ridx));

        let last = choose_num(last_num_idx, last_word_idx, &|l, r| l > r);

        sum += first * 10 + last;
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let file = File::open("input/1.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", first(reader)?);
    println!("{}", second(reader)?);

    Ok(())
}
