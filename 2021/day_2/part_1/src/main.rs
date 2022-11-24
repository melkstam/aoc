use std::io;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let commands: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut split = l.split_whitespace();

            let command = split.next().unwrap();
            let value: i32 = split.next().unwrap().parse().unwrap();

            match command {
                "forward" => Some(Command::Forward(value)),
                "up" => Some(Command::Up(value)),
                "down" => Some(Command::Down(value)),
                _ => None,
            }
        })
        .map(|c| c.unwrap())
        .collect();

    let horizontal = commands.iter().fold(0, |curr, c| {
        curr + match c {
            Command::Forward(val) => *val,
            _ => 0,
        }
    });

    let depth = commands.iter().fold(0, |curr, c| {
        curr + match c {
            Command::Up(val) => -(*val),
            Command::Down(val) => *val,
            _ => 0,
        }
    });

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", horizontal * depth)
}
