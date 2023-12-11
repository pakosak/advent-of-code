use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    pos: Position,
    dir: Dir,
}

impl Command {
    fn new(pos: Position, dir: Dir) -> Self {
        Self { pos, dir }
    }
}

fn parse(reader: BufReader<File>) -> Grid {
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}

fn next_command(prev: Command, grid: &Grid) -> Command {
    let pos = prev.pos;
    match (grid[prev.pos.1][prev.pos.0], prev.dir) {
        ('-', Dir::Left) => Command::new((pos.0 - 1, pos.1), Dir::Left),
        ('-', Dir::Right) => Command::new((pos.0 + 1, pos.1), Dir::Right),
        ('|', Dir::Up) => Command::new((pos.0, pos.1 - 1), Dir::Up),
        ('|', Dir::Down) => Command::new((pos.0, pos.1 + 1), Dir::Down),
        ('L', Dir::Down) => Command::new((pos.0 + 1, pos.1), Dir::Right),
        ('L', Dir::Left) => Command::new((pos.0, pos.1 - 1), Dir::Up),
        ('F', Dir::Up) => Command::new((pos.0 + 1, pos.1), Dir::Right),
        ('F', Dir::Left) => Command::new((pos.0, pos.1 + 1), Dir::Down),
        ('J', Dir::Down) => Command::new((pos.0 - 1, pos.1), Dir::Left),
        ('J', Dir::Right) => Command::new((pos.0, pos.1 - 1), Dir::Up),
        ('7', Dir::Right) => Command::new((pos.0, pos.1 + 1), Dir::Down),
        ('7', Dir::Up) => Command::new((pos.0 - 1, pos.1), Dir::Left),
        (_, _) => panic!("unknown combination"),
    }
}

fn find_start(grid: &Grid) -> Position {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    panic!("no start found");
}

fn first_step(grid: &Grid, start: Position) -> Command {
    if start.1 > 0 {
        let up = grid[start.1 - 1][start.0];
        if up == '|' || up == 'F' || up == '7' {
            return Command::new((start.0, start.1 - 1), Dir::Up);
        }
    }
    if start.1 < grid.len() - 1 {
        let down = grid[start.1 + 1][start.0];
        if down == '|' || down == 'J' || down == 'L' {
            return Command::new((start.0, start.1 + 1), Dir::Down);
        }
    }
    if start.0 > 0 {
        let left = grid[start.1][start.0 - 1];
        if left == '-' || left == 'L' || left == 'F' {
            return Command::new((start.0 - 1, start.1), Dir::Left);
        }
    }
    if start.0 < grid[0].len() - 1 {
        let right = grid[start.1][start.0 + 1];
        if right == '-' || right == '7' || right == 'J' {
            return Command::new((start.0 + 1, start.1), Dir::Right);
        }
    }
    panic!("no first step found");
}

fn compute_path(grid: &Grid) -> HashSet<Position> {
    let start = find_start(&grid);

    let mut current = first_step(&grid, start);
    let mut path = HashSet::new();
    path.insert(current.pos);
    while grid[current.pos.1][current.pos.0] != 'S' {
        println!("{} {:?}", grid[current.pos.1][current.pos.0], current);
        current = next_command(current, &grid);
        path.insert(current.pos);
    }
    path
}

fn first(grid: Grid) -> usize {
    compute_path(&grid).len() / 2
}

fn second(grid: Grid) -> u32 {
    let path = compute_path(&grid);

    let points: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| (0..row.len()).map(|x| !path.contains(&(x, y))).collect())
        .collect();

    for row in &points {}

    1
}

fn main() -> Result<()> {
    let file = File::open("input/10.txt")?;
    let reader = io::BufReader::new(file);

    let grid = parse(reader);

    // println!("{}", first(grid));
    println!("{}", second(grid));

    Ok(())
}
