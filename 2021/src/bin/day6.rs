use std::collections::HashMap;

const DAYS: i64 = 256;

fn bf() {
    let mut fish: Vec<i64> = include_str!("../../input/day6.txt")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    // let mut fish: Vec<i64> = vec![0];

    for day in 0..DAYS {
        fish = fish
            .iter()
            .filter(|f| **f == 0)
            .map(|_| 8)
            .chain(fish.iter().map(|f| match f {
                0 => 6,
                _ => f - 1,
            }))
            .collect();
        // println!("at the end of {} I have {}", day, fish.len());
    }
    // println!("{} {:?}", fish.len(), fish);
    println!("{}", fish.len());
}

fn smart() {
    // println!("{}", include_str!("day6.txt"));

    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();

    fn prod(days: i64, init: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
        if let Some(val) = cache.get(&(days, init)) {
            return *val;
        }
        if days - init < 0 {
            return 0;
        }
        let my_prod = ((days as f32 - init as f32) / 7.0).ceil() as i64;
        // println!("born on {} with {} will produce {}", DAYS - days, init, my_prod);
        let res = my_prod
            + (0..my_prod)
                .map(|fish| prod(days - (init + 1) - fish * 7, 8, cache))
                .sum::<i64>();
        cache.insert((days, init), res);
        return res;
    }

    let fish: Vec<i64> = include_str!("../../input/day6.txt")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .map(|f| 1 + prod(DAYS, f, &mut cache))
        .collect();

    println!("{:?}", fish.iter().sum::<i64>());
    // println!("{}", prod(DAYS, 0));
}

fn main() {
    // bf();
    // println!();
    smart();
}
