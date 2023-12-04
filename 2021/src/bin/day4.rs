use anyhow::Result;
use array2d::Array2D;

const ROW_CNT: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    data: Array2D<i32>,
    marked: Array2D<bool>,
    winning_num: Option<i32>,
}

impl Board {
    fn new(input: Vec<Vec<i32>>) -> Board {
        Board {
            data: Array2D::from_rows(&input),
            marked: Array2D::filled_with(false, ROW_CNT, ROW_CNT),
            winning_num: None,
        }
    }
    fn mark(&mut self, num: i32) -> bool {
        for row_idx in 0..ROW_CNT {
            for col_idx in 0..ROW_CNT {
                if *self.data.get(row_idx, col_idx).unwrap() == num {
                    self.marked.set(row_idx, col_idx, true).unwrap();
                    if self.test_bingo(row_idx, col_idx) {
                        self.winning_num = Some(num);
                        return true;
                    }
                }
            }
        }
        return false;
    }
    fn test_bingo(&self, row: usize, col: usize) -> bool {
        self.test_row(row) || self.test_col(col)
    }
    fn test_row(&self, row: usize) -> bool {
        for col_idx in 0..ROW_CNT {
            if !self.marked.get(row, col_idx).unwrap() {
                return false;
            }
        }
        return true;
    }
    fn test_col(&self, col: usize) -> bool {
        for row_idx in 0..ROW_CNT {
            if !self.marked.get(row_idx, col).unwrap() {
                return false;
            }
        }
        return true;
    }
    fn get_if_unmarked(&self, row: usize, col: usize) -> Option<i32> {
        match self.marked.get(row, col).unwrap() {
            true => None,
            false => Some(*self.data.get(row, col).unwrap()),
        }
    }
    fn get_magic_num(&self) -> i32 {
        let mut sum = 0;
        for row in 0..ROW_CNT {
            for col in 0..ROW_CNT {
                if let Some(maybe) = self.get_if_unmarked(row, col) {
                    sum += maybe;
                }
            }
        }
        sum * self.winning_num.unwrap()
    }
}

fn find_winning_order(boards: &mut Vec<Board>, win_nums: Vec<i32>) -> Vec<usize> {
    let mut winning_order: Vec<usize> = Vec::new();
    for num in win_nums {
        for (idx, board) in boards.iter_mut().enumerate() {
            if (*board).winning_num.is_none() && (*board).mark(num) {
                println!("{} is the winning number for {}", num, idx);
                winning_order.push(idx);
            }
        }
    }
    winning_order
}

fn sup(s: &str) -> Result<i32> {
    Ok(s.parse::<i32>()?)
}

fn main() {
    let mut lines: Vec<_> = include_str!("../../input/day4.txt")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let win_nums: Vec<i32> = lines[0]
        .trim()
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    lines.remove(0);

    let mut boards: Vec<Board> = lines
        .chunks(5)
        .map(|chunk| {
            Board::new(
                chunk
                    .iter()
                    .map(|r| {
                        r.split_whitespace()
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();

    // match sup("ahoj") {
    //     Ok(num) => println!("ok: {}", num),
    //     Err(e) => println!("error: {}", e)
    // }

    // match sup("1") {
    //     Ok(num) => println!("ok: {}", num),
    //     Err(e) => println!("error: {}", e)
    // }

    let winning_order = find_winning_order(&mut boards, win_nums);

    println!("{:?}", winning_order);

    println!(
        "first board: {:?}",
        boards[winning_order[0]].get_magic_num()
    );
    println!(
        "last board: {:?}",
        boards[*winning_order.last().unwrap()].get_magic_num()
    );
}
