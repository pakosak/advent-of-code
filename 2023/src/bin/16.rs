use anyhow::Result;
use std::{collections::HashSet, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct Grid {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let cols = data[0].len();
        let rows = data.len();
        Self { data, rows, cols }
    }

    fn get(&self, pos: Position) -> char {
        self.data[pos.y as usize][pos.x as usize]
    }
    fn invalid_pos(&self, pos: Position) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.cols as isize || pos.y >= self.rows as isize
    }
}

fn step(pos: Position, dir: Dir) -> Position {
    match dir {
        Dir::Up => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Dir::Down => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Dir::Left => Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Dir::Right => Position {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}

fn go(
    mut pos: Position,
    mut dir: Dir,
    grid: &Grid,
    energized: &mut HashSet<Position>,
    visited: &mut HashSet<(Position, Dir)>,
) {
    loop {
        if grid.invalid_pos(pos) || visited.contains(&(pos, dir)) {
            return;
        }
        energized.insert(pos);
        visited.insert((pos, dir));
        match (grid.get(pos), dir) {
            ('.', _) | ('-', Dir::Left) | ('-', Dir::Right) | ('|', Dir::Up) | ('|', Dir::Down) => {
                pos = step(pos, dir);
            }
            ('/', Dir::Left) => {
                dir = Dir::Down;
                pos = step(pos, dir);
            }
            ('/', Dir::Right) => {
                dir = Dir::Up;
                pos = step(pos, dir);
            }
            ('/', Dir::Up) => {
                dir = Dir::Right;
                pos = step(pos, dir);
            }
            ('/', Dir::Down) => {
                dir = Dir::Left;
                pos = step(pos, dir);
            }
            ('\\', Dir::Left) => {
                dir = Dir::Up;
                pos = step(pos, dir);
            }
            ('\\', Dir::Right) => {
                dir = Dir::Down;
                pos = step(pos, dir);
            }
            ('\\', Dir::Up) => {
                dir = Dir::Left;
                pos = step(pos, dir);
            }
            ('\\', Dir::Down) => {
                dir = Dir::Right;
                pos = step(pos, dir);
            }
            ('|', Dir::Left) | ('|', Dir::Right) => {
                go(step(pos, Dir::Up), Dir::Up, grid, energized, visited);
                go(step(pos, Dir::Down), Dir::Down, grid, energized, visited);
                return;
            }
            ('-', Dir::Down) | ('-', Dir::Up) => {
                go(step(pos, Dir::Left), Dir::Left, grid, energized, visited);
                go(step(pos, Dir::Right), Dir::Right, grid, energized, visited);
                return;
            }
            (_, _) => panic!("invalid situation"),
        }
    }
}

fn fire_beam(start_pos: Position, start_dir: Dir, grid: &Grid) -> usize {
    let mut energized: HashSet<Position> = HashSet::new();
    let mut visited: HashSet<(Position, Dir)> = HashSet::new();

    go(start_pos, start_dir, grid, &mut energized, &mut visited);
    energized.len()
}

fn part_one(grid: Grid) -> usize {
    fire_beam(Position { x: 0, y: 0 }, Dir::Right, &grid)
}

fn part_two(grid: Grid) -> usize {
    let starts = (0..grid.rows)
        .map(|row| {
            vec![
                (
                    Position {
                        x: 0,
                        y: row as isize,
                    },
                    Dir::Right,
                ),
                (
                    Position {
                        x: grid.cols as isize - 1,
                        y: row as isize,
                    },
                    Dir::Left,
                ),
            ]
        })
        .chain((0..grid.cols).map(|col| {
            vec![
                (
                    Position {
                        x: col as isize,
                        y: 0,
                    },
                    Dir::Down,
                ),
                (
                    Position {
                        x: col as isize,
                        y: grid.rows as isize - 1,
                    },
                    Dir::Up,
                ),
            ]
        }))
        .flatten();

    starts
        .map(|(pos, dir)| fire_beam(pos, dir, &grid))
        .max()
        .unwrap()
}

fn parse(file_contents: String) -> Grid {
    Grid::new(
        file_contents
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>(),
    )
}

fn main() -> Result<()> {
    let file_contents = fs::read_to_string("input/16.txt")?;
    let grid = parse(file_contents);

    // println!("{}", part_one(grid));
    println!("{}", part_two(grid));

    Ok(())
}
