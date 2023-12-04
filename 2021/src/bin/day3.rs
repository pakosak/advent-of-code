fn pt1() {
    let (gamma, eps) = include_str!("../../input/day3.txt")
        .lines()
        .fold(vec![0; 12], |mut sum, line| {
            for (i, bit) in line.chars().enumerate() {
                sum[i] += bit.to_digit(10).unwrap();
            }
            sum
        })
        .iter()
        .map(|count| (count > &500) as i32)
        .fold((0, 0), |(gamma, eps), bit| {
            (
                (gamma << 1) + (bit == 1) as i32,
                (eps << 1) + (bit == 0) as i32,
            )
        });

    // let tmp = vec![1,0,1]
    //     .iter()
    //     .fold(0, |num, bit| (num << 1) + bit);

    // let gamma = 0;
    // let eps = !gamma;

    println!("{:?}", gamma);
    println!("{:?}", eps);
    println!("{:?}", gamma * eps);
}

fn is_bit_set(num: i32, pos_rev: i32) -> bool {
    let line_len = 12;
    num & (1 << (line_len - pos_rev - 1)) != 0
}

fn collect_nums() -> Vec<i32> {
    include_str!("../../input/day3.txt")
        .lines()
        .map(|l| {
            l.chars()
                .fold(0, |num, bit| (num << 1) + (bit == '1') as i32)
        })
        .collect()
}

fn find(nums: &Vec<i32>, pred: fn(i32, i32) -> bool) -> i32 {
    let line_len = 12;
    let mut bit_idx = 0;

    let mut nums = nums.clone();

    while nums.len() > 1 && bit_idx < line_len {
        let one_occurences: i32 = nums
            .iter()
            .fold(0, |sum, &num| sum + (is_bit_set(num, bit_idx) as i32));
        let should_be_1 = pred(one_occurences * 2, nums.len() as i32);
        // println!("{}. bit should be 1? {}. because {} of {}", bit_idx, should_be_1, one_occurences, nums.len());
        nums.retain(|&num| is_bit_set(num, bit_idx) == should_be_1);
        // println!("filtered {}", nums.len());
        // for x in nums.iter() {
        //     println!("{:b}", x);
        // }
        bit_idx += 1;
    }
    println!("{}", nums[0]);
    nums[0]
}

fn pt2() {
    let nums = collect_nums();

    let a = find(&nums, |occ, len| occ >= len);
    let b = find(&nums, |occ, len| occ < len);

    println!("{} * {} = {}", a, b, a * b);
}

fn main() {
    // pt1();
    pt2();
}
