fn determine(line: &str) -> i32 {
    let s = line.split(" ").collect::<Vec<_>>();
    let mut res = s[1].parse::<i32>().unwrap();
    if s[0] == "up" {
        res *= -1;
    }
    return res;
}

fn pt1() {
    let lines = include_str!("../../input/day2.txt").lines();

    let hor: i32 = lines
        .clone()
        .filter(|line| line.contains("forward"))
        .map(|line| {
            line.split(" ").collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    let depth: i32 = lines
        .filter(|line| !line.contains("forward"))
        .map(|line| determine(line))
        .sum();

    println!("{:?}", hor);
    println!("{:?}", depth);

    println!("{:?}", depth * hor);

    // for x in tmp {
    //     println!("{:?}", x);
    // }
}

fn pt1_() {
    let (d, h) = include_str!("../../input/day2.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(d, h), (dir, val)| {
            match (dir, val.parse::<i32>().unwrap()) {
                ("forward", v) => (d, h + v),
                ("up", v) => (d - v, h),
                ("down", v) => (d + v, h),
                _ => (d, h),
            }
        });

    println!("{} * {} = {}", d, h, d * h);

    // for x in tmp {
    //     println!("{:?}", x);
    // }
}

fn pt2_() {
    let (d, h, _) = include_str!("../../input/day2.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(d, h, a), (dir, val)| {
            match (dir, val.parse::<i32>().unwrap()) {
                ("forward", v) => (d + a * v, h + v, a),
                ("up", v) => (d, h, a - v),
                ("down", v) => (d, h, a + v),
                _ => (d, h, a),
            }
        });

    println!("{} * {} = {}", d, h, d * h);

    // for x in tmp {
    //     println!("{:?}", x);
    // }
}

fn main() {
    // pt1();
    // pt1_();
    pt2_();

    // let a = [1,2,3,4];

    // let b = a
    //     .map(|x| -x)
    //     .iter()
    //     .fold((0, 0), |(h, d), item| (h + item, d + item*2));

    // println!("{:?}", b);

    // let hor = 0;
    // let depth = 0;

    // for line in lines {
    //     let s = line.split(" ");
    // }

    // println!("{:?}", res);
}
