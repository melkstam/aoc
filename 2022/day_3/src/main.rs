use std::io;

fn main() {
    //part1();
    part2();
}

fn get_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        return (c as i32) - ('a' as i32) + 1;
    }

    return (c as i32) - ('A' as i32) + 1 + 26;
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
        .map(|c| get_priority(c))
        .sum();

    println!("{sum}");
}

fn part2() {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();

    let sum: i32 = lines
        .chunks(3)
        .map(|c| {
            for d in alphabet.clone() {
                if c[0].contains(d) && c[1].contains(d) && c[2].contains(d) {
                    return d;
                }
            }

            'a'
        })
        .map(|c| get_priority(c))
        .sum();

    println!("{sum}, {}", ('A'..='z').count());
}
