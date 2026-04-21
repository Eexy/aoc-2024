use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn part_1(reader: impl BufRead) -> i32 {
    let mut total = 0;
    for line in reader.lines() {
        let result = match line {
            Ok(content) => content
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
            Err(e) => {
                eprintln!("unable to read line {e}");
                process::exit(1);
            }
        };

        if result.len() > 1 && result[0] != result[1] {
            let is_increasing = result[0] < result[1];
            let mut is_valid = true;
            for i in 1..result.len() {
                let difference = (result[i] - result[i - 1]).abs();
                if is_increasing
                    && (result[i] <= result[i - 1] || (difference < 1 || difference > 3))
                {
                    is_valid = false;
                } else if !is_increasing
                    && (result[i] > result[i - 1] || (difference < 1 || difference > 3))
                {
                    is_valid = false;
                }
            }

            if is_valid {
                total = total + 1;
            }
        } else if result.len() == 1 {
            total = total + 1;
        }
    }

    total
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
