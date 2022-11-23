use std::io;

fn main() {
    let numbers: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => -1,
                    _ => panic!("Got unkown char '{}'", c),
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    let nbits = numbers[0].len();

    let mut sum = vec![0; nbits];

    numbers.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, s)| sum[i] += *s);
    });

    let bits: Vec<i32> = sum
        .into_iter()
        .map(|val| if val < 0 { 0 } else { 1 })
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    bits.iter().rev().enumerate().for_each(|(i, b)| {
        if *b == 1 {
            gamma = gamma | (1 << i);
        }
    });

    bits.iter().rev().enumerate().for_each(|(i, b)| {
        if *b == 0 {
            epsilon = epsilon | (1 << i);
        }
    });

    println!("{:?}", bits);
    println!("{:?}", gamma);
    println!("{:?}", epsilon);
    println!("{:?}", epsilon * gamma);
}
