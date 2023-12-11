use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse(reader: BufReader<File>) -> Grid {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn compute_expansion_additions(grid: &Grid, expansion_coef: usize) -> (Vec<usize>, Vec<usize>) {
    let mut current_expansions = 0;
    let row_additions = grid
        .iter()
        .map(|row| {
            if row.iter().all(|val| *val == '.') {
                current_expansions += expansion_coef - 1;
            }
            current_expansions
        })
        .collect_vec();

    current_expansions = 0;
    let col_additions = (0..grid[0].len())
        .map(|x| {
            if (0..grid.len()).map(|y| grid[y][x]).all(|val| val == '.') {
                current_expansions += expansion_coef - 1;
            }
            current_expansions
        })
        .collect_vec();

    (row_additions, col_additions)
}

fn dist(a: Pos, b: Pos, row_additions: &[usize], col_additions: &[usize]) -> usize {
    let rows_added = (row_additions[a.y] as i32 - row_additions[b.y] as i32).abs();
    let cols_added = (col_additions[a.x] as i32 - col_additions[b.x] as i32).abs();
    ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as usize
        + rows_added as usize
        + cols_added as usize
}

fn solve(grid: Grid, expansion_coef: usize) -> usize {
    let (row_additions, col_additions) = compute_expansion_additions(&grid, expansion_coef);

    let galaxies: Vec<Pos> = (0..grid.len())
        .flat_map(|y| {
            let grid = &grid;
            (0..grid[y].len()).filter_map(move |x| {
                if grid[y][x] != '.' {
                    Some(Pos { x, y })
                } else {
                    None
                }
            })
        })
        .collect();

    galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .filter(|(a, b)| a != b)
        .map(|(lhs, rhs)| dist(*lhs, *rhs, &row_additions, &col_additions))
        .sum::<usize>()
        / 2
}

fn part_one(grid: Grid) -> usize {
    solve(grid, 1)
}

fn part_two(grid: Grid) -> usize {
    solve(grid, 1_000_000)
}

fn main() -> Result<()> {
    let file = File::open("input/11.txt")?;
    let reader = io::BufReader::new(file);

    let grid = parse(reader);
    // println!("{}", part_one(grid));
    println!("{}", part_two(grid));

    Ok(())
}
