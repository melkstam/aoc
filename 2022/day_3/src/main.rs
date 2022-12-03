use std::io;

fn main() {
    part1();
    // part2();
}

fn part1() {
    let sum: i32 = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (first, last) = l.split_at(l.len() / 2);

            for c1 in first.chars() {
                for c2 in last.chars() {
                    if c1 == c2 {
                        return c1;
                    }
                }
            }

            return 'a';
        })
        .map(|c| {
            if c.is_ascii_lowercase() {
                return (c as i32) - ('a' as i32) + 1;
            }

            return (c as i32) - ('A' as i32) + 1 + 26;
        })
        .sum();

    println!("{sum}");
}
