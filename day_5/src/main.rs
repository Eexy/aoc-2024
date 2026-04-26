use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn is_valid_page(next_pages: &[i32], needed_pages: Option<&Vec<i32>>) -> bool {
    match needed_pages {
        Some(pages) => {
            if pages.iter().any(|page| next_pages.contains(page)) {
                return false;
            }

            true
        }
        None => true,
    }
}

fn is_valid_update(update: &Vec<i32>, pages_order_map: &HashMap<i32, Vec<i32>>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(idx, page)| is_valid_page(&update[idx + 1..], pages_order_map.get(page)))
}

fn correct_update(mut update: Vec<i32>, pages_order_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    while !is_valid_update(&update, pages_order_map) {
        for idx in 0..update.len() {
            let page = update[idx];
            if !is_valid_page(&update[idx + 1..], pages_order_map.get(&page)) {
                match pages_order_map.get(&page) {
                    None => {}
                    Some(needed_pages) => {
                        let result = needed_pages
                            .iter()
                            .filter_map(|needed_pages| {
                                update.iter().position(|x| x == needed_pages)
                            })
                            .max();

                        if let Some(max_position) = result {
                            update.swap(idx, max_position);
                        }
                    }
                }
            }
        }
    }

    // dbg!(&update);

    update
}

fn parse_pages_order(pages_order: Vec<String>) -> HashMap<i32, Vec<i32>> {
    let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();

    for page_order in pages_order.into_iter() {
        let mut content = page_order
            .split("|")
            .filter_map(|value| value.parse::<i32>().ok());

        match (content.next(), content.next()) {
            (Some(previous), Some(page)) => {
                hashmap
                    .entry(page)
                    .and_modify(|value| value.push(previous))
                    .or_insert(vec![previous]);
            }
            _ => {}
        }
    }

    hashmap
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
    let (raw_pages_order, raw_updates): (Vec<String>, Vec<String>) = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() > 0)
        .partition(|line| line.contains("|"));

    let parsed_pages_order = parse_pages_order(raw_pages_order);
    let parsed_updates = raw_updates
        .into_iter()
        .map(|updates| {
            updates
                .split(",")
                .filter_map(|value| value.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // let valid_total: i32 = parsed_updates
    //     .into_iter()
    //     .filter(|update| is_valid_update(&update, &parsed_pages_order))
    //     .map(|update| update[update.len() / 2])
    //     .sum();

    let invalid_total: i32 = parsed_updates
        .into_iter()
        .filter(|update| !is_valid_update(&update, &parsed_pages_order))
        .map(|update| correct_update(update, &parsed_pages_order))
        .map(|update| update[update.len() / 2])
        .sum();

    dbg!(invalid_total);
}
