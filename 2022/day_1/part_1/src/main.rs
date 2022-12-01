use std::{cmp, io};

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut max = 0;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            max = cmp::max(max, sum);
            sum = 0;
            continue;
        }

        let val: i32 = line.parse().unwrap();
        sum += val;
    }

    println!("{}", max);
}
