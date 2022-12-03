use std::io;

fn main() {
    let score = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let chars = s.chars().collect::<Vec<_>>();

            (chars[0], chars[2])
        })
        .fold(0, |acc, g| {
            let mut score = acc;

            score += match g.1 {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!(),
            };

            score += match g.0 {
                'A' => match g.1 {
                    'X' => 3,
                    'Y' => 6,
                    'Z' => 0,
                    _ => panic!(),
                },
                'B' => match g.1 {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => panic!(),
                },
                'C' => match g.1 {
                    'X' => 6,
                    'Y' => 0,
                    'Z' => 3,
                    _ => panic!(),
                },
                _ => panic!(),
            };

            score
        });

    println!("{}", score);
}
