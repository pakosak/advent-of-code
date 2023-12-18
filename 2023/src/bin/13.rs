use anyhow::Result;
use itertools::Itertools;
use std::fs;

type Grid = Vec<Vec<char>>;

fn count_mirrored_differences(col: usize, lines: &Grid) -> usize {
    let line_len = lines[0].len();
    let compared_cnt = (col + 1).min(line_len - col - 1);
    let skipped_cnt = (col + 1).saturating_sub(compared_cnt);
    lines
        .iter()
        .map(|line| {
            line.iter()
                .skip(skipped_cnt)
                .take(compared_cnt)
                .rev()
                .zip(
                    line.iter()
                        .skip(skipped_cnt + compared_cnt)
                        .take(compared_cnt),
                )
                .fold(0, |acc, (a, b)| acc + (a != b) as usize)
        })
        .sum()
}

fn find_mirror_line(grid: &Grid, difference_count: usize) -> Option<usize> {
    (0..(grid[0].len() - 1))
        .filter(|col| count_mirrored_differences(*col, grid) == difference_count)
        .take(1)
        .next()
}

fn compute_mirror_score(grid: Grid, difference_count: usize) -> usize {
    if let Some(mirrored_col) = find_mirror_line(&grid, difference_count) {
        mirrored_col + 1
    } else {
        let rotated = (0..grid[0].len())
            .map(|col| (0..grid.len()).map(|row| grid[row][col]).collect_vec())
            .collect_vec();
        (find_mirror_line(&rotated, difference_count).unwrap() + 1) * 100
    }
}

fn parse_grid(path: &str) -> Grid {
    path.lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec()
}

fn part_one(file_contents: String) -> usize {
    file_contents
        .split("\n\n")
        .map(|blob| compute_mirror_score(parse_grid(blob), 0))
        .sum()
}

fn part_two(file_contents: String) -> usize {
    file_contents
        .split("\n\n")
        .map(|blob| compute_mirror_score(parse_grid(blob), 1))
        .sum()
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input/13.txt")?;
    // println!("{}", part_one(file_contents));
    println!("{}", part_two(file_contents));

    Ok(())
}
