use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn part_1(reader: impl BufRead) -> usize {
    let result = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| !levels.is_empty())
        .filter(|levels| {
            let is_increasing = levels[0] < levels[1];
            levels.windows(2).all(|w| {
                let diff = (w[0] - w[1]).abs();
                let is_valid = if is_increasing {
                    w[0] < w[1]
                } else {
                    w[0] > w[1]
                };

                diff > 0 && diff < 4 && is_valid
            })
        })
        .count();

    // dbg!(&result);

    result
}

fn main() {
    println!("Hello, world!");
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("unable to read file {e}");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let result = part_1(reader);
    println!("{}", result);
}
