use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn part_1(reader: impl BufRead) -> i32 {
    let mut l_list = vec![];
    let mut r_list = vec![];

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(err) => {
                eprintln!("unable to read line: {err}");
                process::exit(1);
            }
        };

        let mut parsed = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().unwrap());

        if let (Some(l_value), Some(r_value)) = (parsed.next(), parsed.next()) {
            l_list.push(l_value);
            r_list.push(r_value);
        }
    }

    l_list.sort();
    r_list.sort();

    l_list.iter().zip(&r_list).map(|(a, b)| (b - a).abs()).sum()
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to read file: {err}");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    dbg!(part_1(reader));
}
