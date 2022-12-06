use std::io;

fn main() {
    //part1();
    part2();
}

fn part1() {
    let mut stacks: [Vec<char>; 9] = [
        vec!['W', 'R', 'F'],
        vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'],
        vec!['P', 'M', 'Z', 'N', 'L'],
        vec!['J', 'C', 'H', 'R'],
        vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'],
        vec!['G', 'C', 'W', 'L', 'F', 'Z'],
        vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'],
        vec!['P', 'N', 'R', 'F', 'W', 'T', 'V', 'C'],
        vec!['J', 'W', 'H', 'G', 'R', 'S', 'V'],
    ];

    io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|f| {
            f.split_whitespace()
                .filter_map(|a| a.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|c| (c[0], (c[1] - 1) as usize, (c[2] - 1) as usize))
        .for_each(|(count, from, to)| {
            for _ in 0..count {
                let to_move = stacks[from].pop().unwrap();
                stacks[to].push(to_move);
            }
        });

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn part2() {
    let mut stacks: [Vec<char>; 9] = [
        vec!['W', 'R', 'F'],
        vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'],
        vec!['P', 'M', 'Z', 'N', 'L'],
        vec!['J', 'C', 'H', 'R'],
        vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'],
        vec!['G', 'C', 'W', 'L', 'F', 'Z'],
        vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'],
        vec!['P', 'N', 'R', 'F', 'W', 'T', 'V', 'C'],
        vec!['J', 'W', 'H', 'G', 'R', 'S', 'V'],
    ];

    io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|f| {
            f.split_whitespace()
                .filter_map(|a| a.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|c| (c[0] as usize, (c[1] - 1) as usize, (c[2] - 1) as usize))
        .for_each(|(count, from, to)| {
            let to_move = stacks[from].split_off(stacks[from].len() - count);
            //println!("{}, {}", to_move.len(), count);
            stacks[to].extend(to_move);
        });

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
