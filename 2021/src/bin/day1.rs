use std::fs;

fn pt1() {
    let res: i32 = fs::read_to_string("day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|x| ((x[1] > x[0]) as i32))
        .sum();

    println!("{}", res);
}

fn pt2() {
    let res: i32 = fs::read_to_string("day1.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|x| ((x[1] > x[0]) as i32))
        .sum();

    println!("{:?}", res);
}

fn main() {

    pt1();
    pt2();

    // let file = File::open("day1.txt").unwrap();

    // let lines: Vec<i32> = io::BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // part2(&lines);
}
