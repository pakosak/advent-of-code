use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Solver {
    get_elem: fn(&[i32]) -> i32,
    op: fn(i32, i32) -> i32,
}

impl Solver {
    fn extrapolate(&self, v: Vec<i32>) -> i32 {
        if v.iter().all(|x| *x == 0) {
            0
        } else {
            (self.op)(
                (self.get_elem)(&v),
                self.extrapolate(v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>()),
            )
        }
    }
}

fn solve(
    reader: BufReader<File>,
    get_elem: fn(&[i32]) -> i32,
    op: fn(i32, i32) -> i32,
) -> Result<i32> {
    let lines = reader.lines();
    let solver = Solver { get_elem, op };
    Ok(lines
        .map(|l| {
            solver.extrapolate(
                l.unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .sum())
}

fn main() -> Result<()> {
    let file = File::open("input/9.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", solve(reader, |v| *v.last().unwrap(), |a, b| a + b)?);
    println!("{}", solve(reader, |v| *v.first().unwrap(), |a, b| a - b)?);

    Ok(())
}
