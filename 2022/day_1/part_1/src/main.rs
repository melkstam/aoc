use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut deeper = 0;
    let mut prev = 0;
    for line in reader.lines() {
        let depth = line.unwrap().parse::<i32>().unwrap();
        if depth > prev && prev != 0 {
            deeper += 1;
        }
        prev = depth;
    }

    println!("{}", deeper);
}
