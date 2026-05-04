use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn generate_combination(combinations: Vec<Vec<char>>, idx: i32, max_size: usize) -> Vec<Vec<char>> {
    if idx == max_size as i32 {
        return combinations;
    }

    let mut new_combinations = vec![];
    for ch in ['+', '*', '|'] {
        for combination in combinations.iter() {
            let mut new_comb = combination.clone();
            new_comb.push(ch);
            new_combinations.push(new_comb);
        }
    }

    generate_combination(new_combinations, idx + 1, max_size)
}

fn is_valid_line(line: &(i64, Vec<i64>)) -> bool {
    if line.1.is_empty() {
        return false;
    }

    // when there is only one number we check that is equal to the goal if not we continue
    if line.1.len() == 1 && line.0 != line.1[0] {
        false;
    }

    // generate all possible combination of operator
    let combinations: Vec<Vec<char>> = generate_combination(vec![vec![]], 0, line.1.len() - 1);

    let mut is_valid = false;

    for combination in combinations {
        let mut temp = line.1[0] as i64;

        // let mut temp_values: Vec<i64> = vec![];
        for idx in 0..combination.len() {
            if combination[idx] == '+' {
                temp += line.1[idx + 1] as i64;
            } else if combination[idx] == '*' {
                temp *= line.1[idx + 1] as i64;
            } else {
                let str = format!("{}{}", temp, line.1[idx + 1]);
                temp = match str.parse::<i64>().ok() {
                    Some(v) => v,
                    None => 0,
                }
            }
        }

        if temp == line.0 {
            is_valid = true;
            break;
        }
    }

    is_valid
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error reading file {err}");
            process::exit(1)
        }
    };

    let reader = BufReader::new(file);
    let lines: Vec<(i64, Vec<i64>)> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            let mut iter = l
                .split([' ', ':'])
                .filter_map(|value| value.trim().parse::<i64>().ok());

            let first = iter.next()?;
            let rest = iter.collect::<Vec<_>>();

            Some((first, rest))
        })
        .collect();

    let result: i64 = lines
        .into_iter()
        .filter(|l| is_valid_line(&l))
        .map(|l| l.0)
        .sum();

    dbg!(result);
}
