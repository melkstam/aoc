use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut values: Vec<i32> = Vec::new();

    let mut lines = io::stdin().lines();

    while let Some(line) = lines.next() {
        let val: i32 = line?.trim().parse().expect("could not parse line");

        values.push(val);
    }

    let mut total: i32 = 0;
    let mut window: [i32; 3] = [0; 3];
    for (i, d) in values.into_iter().enumerate() {
        if window[i % 3] < d && i >= 3 {
            total += 1;
        }

        window[i % 3] = d;
    }

    println!("{total}");

    Ok(())
}
