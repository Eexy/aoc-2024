use std::{
    fmt::format,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
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

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error reading file {err}");
            process::exit(1)
        }
    };

    let reader = BufReader::new(file);
    let values = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            let res = l
                .split(":")
                .map(|value| value.to_owned())
                .collect::<Vec<_>>();

            match (res.first(), res.get(1)) {
                (Some(first), Some(values)) => {
                    let parsed_first = match first.parse::<i64>().ok() {
                        None => return None,
                        Some(v) => v,
                    };

                    let parsed_values = values
                        .split(" ")
                        .map(|v| v.to_owned())
                        .filter_map(|v| v.parse::<i64>().ok())
                        .collect::<Vec<_>>();

                    Some((parsed_first, parsed_values))
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    let mut result: i64 = 0;

    for line in values.into_iter() {
        // we don't check line if there is no combination possible
        if line.1.is_empty() {
            continue;
        }

        // when there is only one number we check that is equal to the goal if not we continue
        if line.1.len() == 1 && line.0 != line.1[0] {
            continue;
        }

        // generate all possible combination of operator
        let combinations: Vec<Vec<char>> = generate_combination(vec![vec![]], 0, line.1.len() - 1);
        // dbg!(&combinations);

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

            // if !temp_values.is_empty() {
            //     temp_values.push(temp);
            // }

            // if combination.contains(&'|') {
            //     dbg!(&combination);
            //     dbg!(&temp_values);
            // }

            // if !temp_values.is_empty() {
            //     temp = match temp_values
            //         .into_iter()
            //         .fold(String::from(""), |acc, v| {
            //             let str = v.to_string();
            //             format!("{}{}", acc, str)
            //         })
            //         .parse::<i64>()
            //         .ok()
            //     {
            //         Some(v) => v,
            //         None => 0,
            //     };
            // }

            if temp == line.0 {
                is_valid = true;
                break;
            }
        }

        if is_valid {
            result += line.0;
        }
    }

    // dbg!(values);
    dbg!(result);
}
