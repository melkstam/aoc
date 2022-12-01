use std::{cmp, io};

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

    /* -------------------------------------------------------------------------- */

    let mut numbers_oxygen: Vec<_> = numbers.clone().into_iter().map(|n| n.clone()).collect();

    for i in 0..nbits {
        let mut counter = 0;
        numbers_oxygen.iter().for_each(|row| counter += row[i]);

        if counter == 0 {
            counter = 1;
        }

        let most_common = counter / counter.abs();

        numbers_oxygen = numbers_oxygen
            .into_iter()
            .filter(|num| num[i] == most_common)
            .collect();

        if numbers_oxygen.len() == 1 {
            break;
        }
    }

    let bits_oxygen = numbers_oxygen[0].clone();

    let mut oxygen = 0;

    bits_oxygen
        .iter()
        .map(|b| cmp::max(b, &0))
        .rev()
        .enumerate()
        .for_each(|(i, b)| {
            oxygen = oxygen | (b << i);
        });

    /* -------------------------------------------------------------------------- */

    let mut numbers_co2: Vec<_> = numbers.clone().into_iter().map(|n| n.clone()).collect();

    for i in 0..nbits {
        let mut counter = 0;
        numbers_co2.iter().for_each(|row| counter += row[i]);

        if counter == 0 {
            counter = -1;
        }

        let most_common = counter / counter.abs();

        numbers_co2 = numbers_co2
            .into_iter()
            .filter(|num| num[i] == most_common)
            .collect();

        if numbers_co2.len() == 1 {
            break;
        }
    }

    let bits_co2 = numbers_co2[0].clone();

    let mut co2 = 0;

    bits_co2
        .iter()
        .map(|b| cmp::max(b, &0))
        .rev()
        .enumerate()
        .for_each(|(i, b)| {
            co2 = co2 | (b << i);
        });

    /* -------------------------------------------------------------------------- */
    println!(
        "{:?}",
        bits_oxygen
            .into_iter()
            .rev()
            .map(|b| cmp::max(b, 0))
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    println!(
        "{:?}",
        bits_co2
            .into_iter()
            .rev()
            .map(|b| cmp::max(b, 0))
            .collect::<Vec<i32>>()
    );
    println!("{:?}", oxygen);
    println!("{:?}", co2);
    println!("{:?}", oxygen * co2);
}
