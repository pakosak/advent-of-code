use std::collections::HashMap;
fn main() {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    include_str!("../../input/day5.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|point| {
                    point
                        .split(",")
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        // .filter(|line| line[0][0] != line[1][0] && line[0][1] != line[1][1])
        .for_each(|line| {
            let (x1, y1, x2, y2) = (line[0][0], line[0][1], line[1][0], line[1][1]);
            let x_act = if x1 < x2 {
                1
            } else if x1 > x2 {
                -1
            } else {
                0
            };
            let y_act = if y1 < y2 {
                1
            } else if y1 > y2 {
                -1
            } else {
                0
            };

            let mut x = x1;
            let mut y = y1;
            // println!("going {} {} -> {} {}", x1, y1, x2, y2);
            while x != x2 || y != y2 {
                // println!("marking {} {}", x, y);
                *map.entry((x, y)).or_insert(0) += 1;
                x = x + x_act;
                y = y + y_act;
            }
            *map.entry((x, y)).or_insert(0) += 1;
        });

    println!(
        "{:?}",
        map.iter()
            .filter(|(_, v)| v > &&1)
            .collect::<Vec<_>>()
            .len()
    );

    // let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    // map.insert((4,5), 3);

    // *map.entry((4,4)).or_insert(0) += 1;

    // for i in 10..0 {
    //     println!("{}", i);
    // }

    // let v = vec![0,1,2,3];
    // let m: Vec<_> = v.iter().map(|x| x*2).collect();

    // println!("{:?}", map);
}
