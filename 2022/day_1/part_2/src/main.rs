use std::io;

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();

    let mut vals: Vec<i32> = lines
        .split(|l| l.is_empty())
        .map(|e| e.iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap()))
        .collect();

    vals.sort();
    let max_3: i32 = vals.iter().rev().take(3).sum();
    println!("{}", max_3);
}
