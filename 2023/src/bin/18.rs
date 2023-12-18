use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs};

type Grid = Vec<Vec<bool>>;

fn step(pos: (usize, usize), dir: char, grid: &Grid) -> Option<(usize, usize)> {
    match dir {
        'U' => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        'D' => {
            if pos.1 == grid.len() - 1 {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
        'L' => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        'R' => {
            if pos.0 == grid[0].len() - 1 {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        _ => panic!("invalid direction"),
    }
}

fn num_char_to_dir(num: char) -> char {
    match num {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("invalid direction"),
    }
}

fn compute_inner_area(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![(0, 0)];
    while let Some(pos) = queue.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        for dir in ['U', 'D', 'L', 'R'].iter() {
            if let Some(next) = step(pos, *dir, grid) {
                if !grid[next.1][next.0] {
                    queue.push(next);
                }
            }
        }
    }
    grid[0].len() * grid.len() - visited.len()
}

fn prepare_grid(commands: &[(char, i32)]) -> (Grid, (usize, usize)) {
    let intermediate_positions: Vec<(i32, i32)> = commands
        .iter()
        .scan((0_i32, 0_i32), |acc, (dir, steps)| {
            if *dir == 'R' {
                acc.0 += *steps;
            } else if *dir == 'L' {
                acc.0 -= *steps;
            } else if *dir == 'D' {
                acc.1 += *steps;
            } else if *dir == 'U' {
                acc.1 -= *steps;
            }
            Some(*acc)
        })
        .collect();
    let rightmost = intermediate_positions.iter().map(|p| p.0).max().unwrap();
    let leftmost = intermediate_positions.iter().map(|p| p.0).min().unwrap();
    let downmost = intermediate_positions.iter().map(|p| p.1).max().unwrap();
    let upmost = intermediate_positions.iter().map(|p| p.1).min().unwrap();
    println!(
        "rightmost: {}, leftmost: {}, downmost: {}, upmost: {}",
        rightmost, leftmost, downmost, upmost
    );
    let horizontal_size = (rightmost - leftmost) as usize;
    let vertical_size = (downmost - upmost) as usize;
    let mut grid = vec![vec![false; horizontal_size + 3]; vertical_size + 3];

    let current = (
        leftmost.unsigned_abs() as usize + 1,
        upmost.unsigned_abs() as usize + 1,
    );
    grid[current.1][current.0] = true;
    (grid, current)
}

fn compute_trench_area(commands: Vec<(char, i32)>) -> usize {
    let (mut grid, mut current) = prepare_grid(&commands);
    for (dir, steps) in commands {
        for _ in 0..steps {
            current = step(current, dir, &grid).unwrap();
            grid[current.1][current.0] = true;
        }
    }
    let pretty = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", pretty);
    compute_inner_area(&grid)
}

fn part_one(file_contents: String) -> usize {
    compute_trench_area(
        file_contents
            .lines()
            .map(|line| {
                let mut spl = line.split_whitespace();
                let dir = spl.next().unwrap().chars().next().unwrap();
                let steps = spl.next().unwrap().parse::<i32>().unwrap();
                (dir, steps)
            })
            .collect(),
    )
}

fn part_two(file_contents: String) -> usize {
    compute_trench_area(
        file_contents
            .lines()
            .map(|line| {
                let hex = line.split_once('#').unwrap().1.trim_end_matches(')');
                let dir = num_char_to_dir(hex.chars().last().unwrap());
                let steps = i64::from_str_radix(&hex[..6], 16).unwrap() as i32;
                (dir, steps)
            })
            .collect(),
    )
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input/18.txt")?;

    println!("{}", part_one(file_contents));
    // println!("{}", part_two(file_contents));

    Ok(())
}
