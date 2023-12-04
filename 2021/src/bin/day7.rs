fn main() {
    let mut nums: Vec<i64> = include_str!("../../input/day7.txt")
        .trim()
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    nums.sort();
    let median = nums[nums.len() / 2];

    fn cost(num: i64, base: i64) -> i64 {
        (1..((num - base).abs() + 1)).sum()
    }

    fn whole_cost(v: &Vec<i64>, base: i64) -> i64 {
        v.iter().fold(0, |acc, &num| acc + cost(num, base))
    }

    let res = (median - 100..median + 300)
        // let res = (0..*nums.last().unwrap())
        .map(|x| whole_cost(&nums, x))
        .min()
        .unwrap();

    println!("{}", res);

    // for x in (median-100..median+300) {
    //     let res = whole_cost(&nums, x);
    //     println!("{}: {}", x, res);
    // }

    println!(
        "{}",
        whole_cost(&nums, nums.iter().sum::<i64>() / (nums.len() as i64))
    );
}
