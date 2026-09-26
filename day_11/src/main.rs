use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn extract_leading_zeroes(slice: &str) -> String {
    match slice.trim_start_matches('0') {
        "" => "0".to_string(),
        rest => rest.to_string(),
    }
}

fn split_stones(
    stones: HashMap<String, i64>,
    current_blink: usize,
    max_blink: usize,
) -> HashMap<String, i64> {
    if current_blink == max_blink {
        return stones;
    }

    let mut new_stones: HashMap<String, i64> = HashMap::new();

    for (entry, val) in stones.into_iter() {
        if val == 0 {
            continue;
        }

        if entry == "0" {
            new_stones
                .entry("1".to_string())
                .and_modify(|e| *e += val)
                .or_insert(val);
        } else if entry.len() % 2 == 0 {
            let mid = entry.len() / 2;
            let lslice = extract_leading_zeroes(&entry[0..mid]);
            let rslice = extract_leading_zeroes(&entry[mid..]);

            new_stones
                .entry(lslice)
                .and_modify(|e| *e += val)
                .or_insert(val);
            new_stones
                .entry(rslice)
                .and_modify(|e| *e += val)
                .or_insert(val);
        } else {
            let new_entry = entry
                .parse::<i64>()
                .map_or(0.to_string(), |v| (v * 2024).to_string());
            new_stones
                .entry(new_entry)
                .and_modify(|e| *e += val)
                .or_insert(val);
        }
    }

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

    let mut stones_hashmap: HashMap<String, i64> = HashMap::new();
    for stone in stones.into_iter() {
        stones_hashmap
            .entry(stone)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    let result_stones = split_stones(stones_hashmap, 0, 75);
    dbg!(result_stones.into_values().fold(0i64, |acc, val| acc + val));
    Ok(())
}
