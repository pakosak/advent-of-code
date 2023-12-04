use std::collections::HashSet;
use std::iter::FromIterator;

fn pt1() {
    let wanted = HashSet::from([2, 3, 4, 7]);

    let suma: i32 = include_str!("../../input/day8.txt")
        .lines()
        .map(|l| {
            l.split_once('|')
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .filter(|code| wanted.contains(&code.len()))
                .count() as i32
        })
        .sum();
    println!("{:?}", suma);
}

fn main() {
    let vals: Vec<Vec<HashSet<char>>> = include_str!("../../input/day8.txt")
        .lines()
        .map(|l| {
            l.split('|')
                .map(|x| {
                    x.trim()
                        .split_whitespace()
                        .map(|code| HashSet::from_iter(code.chars()))
                        .collect()
                })
                .collect()
        })
        .collect();

    let val = &vals[0];
    // let ch: Vec<char> = val[0][0].chars().collect::<Vec<char>>();
    // let setik: HashSet<&char> = HashSet::from_iter(ch.iter());
    // let setik2: HashSet<char> = HashSet::from_iter(val[0][0].chars());
    println!("{:?}", val);
    // println!("{:?} {:?} {:?}", val, ch, setik2);
}
