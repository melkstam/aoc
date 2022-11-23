use std::io;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug)]
struct State {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    let commands = io::stdin()
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
        .map(|c| c.unwrap());

    let mut state = State {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    commands.for_each(|c| match c {
        Command::Up(val) => {
            state.aim -= val;
        }
        Command::Down(val) => {
            state.aim += val;
        }
        Command::Forward(val) => {
            state.horizontal += val;
            state.depth += state.aim * val;
        }
    });

    println!("State: {:?}", state);
    println!("Multiplied: {}", state.horizontal * state.depth)
}
