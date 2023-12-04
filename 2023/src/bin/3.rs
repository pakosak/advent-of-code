use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Num {
    start: usize,
    end: usize,
    line: usize,
    val: u32,
}

fn is_special_close(num: &Num, specials: &[(usize, usize)]) -> bool {
    for line_idx in
        num.line.checked_sub(1).unwrap_or(num.line)..=num.line.checked_add(1).unwrap_or(num.line)
    {
        if (num.start.checked_sub(1).unwrap_or(num.start)..=num.end)
            .map(|idx| specials.contains(&(line_idx, idx)))
            .any(|x| x)
        {
            println!("{}", num.val);
            return true;
        }
    }
    false
}

fn first(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut special: Vec<(usize, usize)> = vec![];
    let mut nums: Vec<Num> = vec![];

    for (line_idx, line) in lines.enumerate() {
        let line = line?;

        let mut num = 0;
        let mut num_start: Option<usize> = None;
        for (idx, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                num = num * 10 + ch.to_digit(10).unwrap();
                if num_start.is_none() {
                    num_start = Some(idx);
                }
            } else {
                if let Some(start) = num_start {
                    nums.push(Num {
                        start,
                        end: idx,
                        line: line_idx,
                        val: num,
                    });
                    num_start = None;
                    num = 0;
                }
                if ch != '.' {
                    special.push((line_idx, idx));
                }
            }
        }
        if let Some(last_start) = num_start {
            nums.push(Num {
                start: last_start,
                end: line.len(),
                line: line_idx,
                val: num,
            });
        }
    }

    println!("{:?}", special);
    println!("{:?}", nums);

    Ok(nums
        .iter()
        .filter(|num| is_special_close(num, &special))
        .fold(0, |acc, num| acc + num.val))
}

fn filter_specials(nums: &mut [Num], specials: Vec<(usize, usize)>) -> u32 {
    let mut true_specials: HashMap<(usize, usize), u32> = HashMap::new();
    let mut true_specials_vals: HashMap<(usize, usize), u32> = HashMap::new();

    for num in nums {
        for line_idx in num.line.checked_sub(1).unwrap_or(num.line)
            ..=num.line.checked_add(1).unwrap_or(num.line)
        {
            for special in (num.start.checked_sub(1).unwrap_or(num.start)..=num.end)
                .filter(|idx| specials.contains(&(line_idx, *idx)))
            {
                *true_specials.entry((line_idx, special)).or_insert(0) += 1;
                *true_specials_vals.entry((line_idx, special)).or_insert(1) *= num.val;
            }
        }
    }
    true_specials_vals
        .into_iter()
        .filter_map(|(k, v)| {
            if true_specials[&k] == 2 {
                Some(v)
            } else {
                None
            }
        })
        .sum()
}

fn second(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut special: Vec<(usize, usize)> = vec![];
    let mut nums: Vec<Num> = vec![];

    for (line_idx, line) in lines.enumerate() {
        let line = line?;

        let mut num = 0;
        let mut num_start: Option<usize> = None;
        for (idx, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                num = num * 10 + ch.to_digit(10).unwrap();
                if num_start.is_none() {
                    num_start = Some(idx);
                }
            } else {
                if let Some(start) = num_start {
                    nums.push(Num {
                        start,
                        end: idx,
                        line: line_idx,
                        val: num,
                    });
                    num_start = None;
                    num = 0;
                }
                if ch == '*' {
                    special.push((line_idx, idx));
                }
            }
        }
        if let Some(last_start) = num_start {
            nums.push(Num {
                start: last_start,
                end: line.len(),
                line: line_idx,
                val: num,
            });
        }
    }

    // println!("{:?}", special);
    // println!("{:?}", nums);

    Ok(filter_specials(&mut nums, special))
}

fn main() -> Result<()> {
    let file = File::open("input/3.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", first(reader)?);
    println!("{}", second(reader)?);

    Ok(())
}
