use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    part_1().unwrap();
}

fn part_1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let res: u32 = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let lindex = line.find(char::is_numeric).unwrap();
            let rindex = line.rfind(char::is_numeric).unwrap();

            let a = line.chars().nth(lindex).unwrap().to_digit(10).unwrap();
            let b = line.chars().nth(rindex).unwrap().to_digit(10).unwrap();

            a * 10 + b
        })
        .sum();

    println!("{:?}", res);

    Ok(())
}
