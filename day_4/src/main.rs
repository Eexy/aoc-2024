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

    let diagonal_iter = content.iter().enumerate();
    for (index, line) in diagonal_iter {
        let iter = line.iter().enumerate();
        for (i, ch) in iter {
            if ch == &'A' {
                let is_valid_height_range = index > 0 && index < content.len() - 1;
                let is_valid_width_range = i > 0 && i < line.len() - 1;

                if is_valid_width_range && is_valid_height_range {
                    // diag top left to bottom right
                    let first_lhs_ch = content[index - 1][i - 1];
                    let last_lhs_ch = content[index + 1][i + 1];

                    let lhs = (first_lhs_ch == 'S' && last_lhs_ch == 'M')
                        || (first_lhs_ch == 'M' && last_lhs_ch == 'S');

                    //diag top right to bottom left
                    let first_rhs_ch = content[index - 1][i + 1];
                    let last_rhs_ch = content[index + 1][i - 1];

                    let rhs = (first_rhs_ch == 'S' && last_rhs_ch == 'M')
                        || (first_rhs_ch == 'M' && last_rhs_ch == 'S');

                    if rhs && lhs {
                        total += 1;
                    }
                }
            }
        }
    }
    dbg!(total);
}
