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
        return false;
    }

    // generate all possible combination of operator
    // let combinations: Vec<Vec<char>> = generate_combination(vec![vec![]], 0, line.1.len() - 1);

    // number of operator slots between the numbers
    let n_operator = line.1.len() as u32 - 1;
    // each slot has 3 choices (+, *, |), so total combinations is 3^n
    let n_combinations = 3_i64.pow(n_operator);

    // each integer i from 0..n_combinations encodes one full operator sequence
    // its base-3 digits map to operators: 0→+, 1→*, 2→|
    for i in 0..n_combinations {
        let mut temp = line.1[0];

        for idx in 0..n_operator as usize {
            let next_operand = line.1[idx + 1];
            // extract the operator at position idx by shifting right in base-3
            // e.g. i=5 (base-3: 0,1,2) with 3 operator slots:
            //   idx=0: (5 / 3^0) % 3 = (5 / 1) % 3 = 5 % 3 = 2 → |
            //   idx=1: (5 / 3^1) % 3 = (5 / 3) % 3 = 1 % 3 = 1 → *
            //   idx=2: (5 / 3^2) % 3 = (5 / 9) % 3 = 0 % 3 = 0 → +
            // dividing by 3^idx shifts the base-3 digits right so the digit
            // at position idx lands at the units place, then % 3 isolates it
            match (i / 3_i64.pow(idx as u32)) % 3 {
                0 => temp += next_operand,
                1 => temp *= next_operand,
                2 => {
                    let str = format!("{}{}", temp, next_operand);
                    temp = match str.parse::<i64>().ok() {
                        Some(v) => v,
                        None => 0,
                    };
                }
                _ => unreachable!(),
            };
        }

        if temp == line.0 {
            return true;
        }
    }

    false
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
