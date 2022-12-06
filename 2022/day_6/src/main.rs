use std::io;

fn main() {
    //part1();
    part2();
}

fn part1() {
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let message: Vec<char> = m.chars().collect();

    let first = first_unique_window(&message, 4);
    println!("{first}");
}

fn part2() {
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let message: Vec<char> = m.chars().collect();

    let first = first_unique_window(&message, 14);
    println!("{first}");
}

fn first_unique_window<T: std::cmp::PartialEq>(list: &[T], size: usize) -> usize {
    for (i, win) in list.windows(size).enumerate() {
        if is_unique(win) {
            return i + size;
        }
    }

    return 0;
}

fn is_unique<T: std::cmp::PartialEq>(a: &[T]) -> bool {
    for i in 0..a.len() {
        for j in (i + 1)..a.len() {
            if a[i] == a[j] {
                return false;
            }
        }
    }

    return true;
}
