use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn max_cnt_by_color(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("Unknown color"),
    }
}

fn first(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut sum = 0;
    for (idx, line) in lines.enumerate() {
        let line = line?;

        let (_, games) = line.split_once(':').unwrap();
        if games
            .split(';')
            .map(|game| {
                game.split(',')
                    .map(|ball_info| {
                        let (num, color) = ball_info.trim().split_once(' ').unwrap();
                        num.parse::<u32>().unwrap() <= max_cnt_by_color(color)
                    })
                    .all(|x| x)
            })
            .all(|x| x)
        {
            sum += (idx + 1) as u32;
        }
    }
    Ok(sum)
}

fn max_ball_num_by_color(input_color: &str, ball_counts: &[(u32, String)]) -> u32 {
    ball_counts
        .iter()
        .rev()
        .filter(|(_, color)| color == input_color)
        .take(1)
        .next()
        .unwrap()
        .0
}

fn second(reader: BufReader<File>) -> Result<u32> {
    let lines = reader.lines();

    let mut sum = 0;
    for line in lines {
        let line = line?;

        let (_, games) = line.split_once(':').unwrap();

        let mut ball_counts = games
            .split(';')
            .flat_map(|game| {
                game.split(',')
                    .map(|ball_info| {
                        let (num, color) = ball_info.trim().split_once(' ').unwrap();
                        (num.parse::<u32>().unwrap(), color.to_string())
                    })
                    .collect::<Vec<(u32, String)>>()
            })
            .collect::<Vec<(u32, String)>>();

        ball_counts.sort_by(|(lhs_num, _), (rhs_num, _)| lhs_num.cmp(rhs_num));
        let blue = max_ball_num_by_color("blue", &ball_counts);
        let red = max_ball_num_by_color("red", &ball_counts);
        let green = max_ball_num_by_color("green", &ball_counts);
        // println!("{} {} {}", blue, red, green);

        sum += blue * red * green;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let file = File::open("input/2.txt")?;
    let reader = io::BufReader::new(file);

    // println!("{}", first(reader)?);
    println!("{}", second(reader)?);

    Ok(())
}
