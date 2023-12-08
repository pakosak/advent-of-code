use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Graph = HashMap<String, (String, String)>;

fn parse(reader: BufReader<File>) -> (Graph, String) {
    let mut lines = reader.lines();
    let commands = lines.next().unwrap().unwrap();
    lines.next();
    let graph = lines
        .map(|l| {
            let l = l.unwrap();
            let (src, dsts) = l.split_once('=').unwrap();
            let src = src.trim().to_string();
            let (left, right) = dsts.split_once(',').unwrap();
            let left = left.trim().trim_start_matches('(').to_string();
            let right = right.trim_end_matches(')').trim().to_string();
            (src, (left, right))
        })
        .fold(HashMap::new(), |mut acc, (src, dsts)| {
            acc.insert(src, dsts);
            acc
        });
    (graph, commands)
}

fn first(graph: Graph, commands: String) -> Result<u32> {
    let mut it = commands.chars().cycle();

    let mut current = "AAA";
    let mut count: u32 = 0;

    while current != "ZZZ" {
        let command = it.next().unwrap();
        let (left, right) = graph.get(current).unwrap();

        current = match command {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown command: {}", command),
        };
        count += 1;
    }
    Ok(count)
}

fn get_period(graph: &Graph, commands: &str, start: String) -> u64 {
    let mut it = commands.chars().cycle();
    let mut current = start;
    let mut count: u64 = 0;

    while !current.ends_with('Z') {
        let command = it.next().unwrap();
        let (left, right) = graph.get(&current).unwrap();

        current = match command {
            'L' => left.to_owned(),
            'R' => right.to_owned(),
            _ => panic!("Unknown command: {}", command),
        };
        count += 1;
    }
    count
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != b {
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn second(graph: Graph, commands: String) -> Result<u64> {
    Ok(graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|aa| get_period(&graph, &commands, aa.to_string()))
        .fold(1, lcm))
}

fn main() -> Result<()> {
    let file = File::open("input/8.txt")?;
    let reader = io::BufReader::new(file);

    let (graph, commands) = parse(reader);
    // println!("{}", first(graph, commands)?);
    println!("{}", second(graph, commands)?);

    Ok(())
}
