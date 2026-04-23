use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error reading file {err}");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let content = reader
        .lines()
        .filter_map(|line| line.ok().map(|line| line.chars().collect::<Vec<_>>()))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut total = 0;
    // search horizontally
    let horizontal_foward_iter = content.iter().enumerate();
    for (index, line) in horizontal_foward_iter {
        let iter = line.iter().enumerate();
        for (i, ch) in iter {
            if ch == &'X' {
                let forward_str = line
                    .get(i..=i + 3)
                    .map(|s| s.iter().collect::<String>())
                    .unwrap_or(String::from(""));
                if forward_str == "XMAS" {
                    total += 1;
                }

                if i >= 3 {
                    let backward_str = line
                        .get(i - 3..=i)
                        .map(|s| s.iter().collect::<String>())
                        .unwrap_or(String::from(""));
                    if backward_str == "SAMX" {
                        total += 1;
                    }
                }
            }
        }
    }
    // search vertical
    let vertical_iter = content.iter().enumerate();
    for (index, line) in vertical_iter {
        let iter = line.iter().enumerate();
        for (i, ch) in iter {
            if ch == &'X' {
                if index > 2 {
                    let mut top_str = String::from("X");
                    top_str.push(content[index - 1][i]);
                    top_str.push(content[index - 2][i]);
                    top_str.push(content[index - 3][i]);

                    if top_str == "XMAS" {
                        total += 1;
                    }
                }

                if index < content.len() - 3 {
                    let mut bottom_str = String::from("X");
                    bottom_str.push(content[index + 1][i]);
                    bottom_str.push(content[index + 2][i]);
                    bottom_str.push(content[index + 3][i]);

                    if bottom_str == "XMAS" {
                        total += 1;
                    }
                }
            }
        }
    }
    // search diagonal
    let diagonal_iter = content.iter().enumerate();
    for (index, line) in diagonal_iter {
        let iter = line.iter().enumerate();
        for (i, ch) in iter {
            // let is_valid_diag_top_right = index - 3 >= 0 && i + 3 > line.len();
            // let is_valid_diag_top_left = index > 2 && index - 3 <= 0 && i - 3 >= 0;
            // let is_valid_diag_bottom_left = index + 3 < content.len() && i - 3 >= 0;
            //diag bottom righ
            if ch == &'X' {
                let is_valid_diag_bottom_right = index + 3 < content.len() && i + 3 < line.len();
                if is_valid_diag_bottom_right {
                    let mut diag_bottom_right_str = String::from("X");
                    diag_bottom_right_str.push(content[index + 1][i + 1]);
                    diag_bottom_right_str.push(content[index + 2][i + 2]);
                    diag_bottom_right_str.push(content[index + 3][i + 3]);

                    if diag_bottom_right_str == "XMAS" {
                        total += 1;
                    }
                }

                let is_valid_diag_top_right = index > 2 && index - 3 >= 0 && i + 3 < line.len();
                if is_valid_diag_top_right {
                    let mut diag_top_right_str = String::from("X");
                    diag_top_right_str.push(content[index - 1][i + 1]);
                    diag_top_right_str.push(content[index - 2][i + 2]);
                    diag_top_right_str.push(content[index - 3][i + 3]);

                    if diag_top_right_str == "XMAS" {
                        total += 1;
                    }
                }

                let is_valid_diag_top_left = index > 2 && index - 3 >= 0 && i > 2 && i - 3 >= 0;
                if is_valid_diag_top_left {
                    let mut diag_top_left_str = String::from("X");
                    diag_top_left_str.push(content[index - 1][i - 1]);
                    diag_top_left_str.push(content[index - 2][i - 2]);
                    diag_top_left_str.push(content[index - 3][i - 3]);

                    if diag_top_left_str == "XMAS" {
                        total += 1;
                    }
                }

                let is_valid_diag_bottom_left = index + 3 < content.len() && i > 2 && i - 3 >= 0;
                if is_valid_diag_bottom_left {
                    let mut diag_top_left_str = String::from("X");
                    diag_top_left_str.push(content[index + 1][i - 1]);
                    diag_top_left_str.push(content[index + 2][i - 2]);
                    diag_top_left_str.push(content[index + 3][i - 3]);

                    if diag_top_left_str == "XMAS" {
                        total += 1;
                    }
                }
            }
        }
    }
    dbg!(total);
}
