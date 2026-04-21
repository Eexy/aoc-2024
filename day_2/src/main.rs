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

fn is_valid_slice(slice: &[i32]) -> bool {
    let is_increasing = slice[0] < slice[1];
    slice.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        let is_valid = if is_increasing {
            w[0] < w[1]
        } else {
            w[0] > w[1]
        };

        diff > 0 && diff < 4 && is_valid
    })
}

fn try_for_each_slice(slice: &[i32]) -> bool {
    (0..slice.len()).any(|i| {
        let sub = (0..slice.len())
            .filter(|&j| j != i)
            .map(|j| slice[j])
            .collect::<Vec<_>>();

        is_valid_slice(&sub)
    })
}

fn part_2(reader: impl BufRead) -> usize {
    let result = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| !levels.is_empty())
        .filter(|levels| is_valid_slice(levels) || try_for_each_slice(levels))
        .count();

    // dbg!(&result);

    result
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("unable to read file {e}");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let result = part_2(reader);
    println!("{}", result);
}
