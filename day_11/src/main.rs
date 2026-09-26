use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn extract_leading_zeroes(slice: &str) -> String {
    match slice.trim_start_matches('0') {
        "" => "0".to_string(),
        rest => rest.to_string(),
    }
    // match stone.chars().position(|ch| ch != '0') {
    //     None => "0".to_string(),
    //     Some(val) => stone[val..].to_string(),
    // }
}

fn split_stones(stones: Vec<String>, current_blink: usize, max_blink: usize) -> Vec<String> {
    if current_blink == max_blink {
        return stones;
    }

    let new_stones = stones
        .into_iter()
        .flat_map(|stone| {
            if stone == "0".to_string() {
                return vec!["1".to_string()];
            }

            if stone.len() % 2 == 0 {
                let mid = stone.len() / 2;
                let rslice = extract_leading_zeroes(&stone[0..mid]);
                let lslice = extract_leading_zeroes(&stone[mid..]);

                return vec![rslice, lslice];
            }

            vec![
                stone
                    .parse::<i64>()
                    .map_or(0.to_string(), |v| (v * 2024).to_string()),
            ]
        })
        .collect::<Vec<_>>();

    split_stones(new_stones, current_blink + 1, max_blink)
}

fn main() -> Result<(), String> {
    let file = File::open("input.txt").map_err(|err| format!("unable to read file {err}"))?;
    let reader = BufReader::new(file);
    let stones = reader
        .lines()
        .next()
        .ok_or_else(|| "input file is empty".to_string())?
        .map_err(|err| format!("unable to read line: {err}"))?
        .split(" ")
        .map(|value| value.to_string())
        .collect::<Vec<_>>();

    dbg!(&stones);

    let result_stones = split_stones(stones, 0, 25);
    dbg!(result_stones.len());

    Ok(())
}
