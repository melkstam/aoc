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

    let res = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (game_tag, game) = line.split_once(": ").unwrap();

            let valid = game
                .split(&[',', ';'][..])
                .map(|pull| pull.trim())
                .map(|pull| pull.split_once(" ").unwrap())
                .map(|(c, color)| (c.parse::<u32>().unwrap(), color))
                .all(|(c, color)| {
                    let max = match color {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => 0,
                    };

                    c <= max
                });

            if !valid {
                return 0;
            }

            game_tag
                .trim_matches(|c: char| !c.is_numeric())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();

    println!("{:?}", res);

    Ok(())
}
