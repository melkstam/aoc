use std::io;

fn main() {
    part2();
}

fn part1() {
    let count = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (r1, r2) = l.split_once(",").unwrap();

            let (r1_start, r1_end) = r1.split_once("-").unwrap();
            let (r2_start, r2_end) = r2.split_once("-").unwrap();

            let r1_s: i32 = r1_start.parse().unwrap();
            let r1_e: i32 = r1_end.parse().unwrap();
            let r2_s: i32 = r2_start.parse().unwrap();
            let r2_e: i32 = r2_end.parse().unwrap();

            ((r1_s..=r1_e), (r2_s..=r2_e))
        })
        .filter(|r| {
            (r.0.start() >= r.1.start() && r.0.end() <= r.1.end())
                || (r.0.start() <= r.1.start() && r.0.end() >= r.1.end())
        })
        .count();

    println!("{}", count);
}

fn part2() {
    let count = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (r1, r2) = l.split_once(",").unwrap();

            let (r1_start, r1_end) = r1.split_once("-").unwrap();
            let (r2_start, r2_end) = r2.split_once("-").unwrap();

            let r1_s: i32 = r1_start.parse().unwrap();
            let r1_e: i32 = r1_end.parse().unwrap();
            let r2_s: i32 = r2_start.parse().unwrap();
            let r2_e: i32 = r2_end.parse().unwrap();

            ((r1_s..=r1_e), (r2_s..=r2_e))
        })
        .filter(|r| (r.0.start() <= r.1.end()) && (r.0.end() >= r.1.start()))
        .count();

    println!("{}", count);
}
