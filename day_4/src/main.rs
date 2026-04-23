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

    for col in 1..content.len() - 1 {
        let line = &content[col];
        for row in 1..line.len() - 1 {
            let ch = line[row];
            if ch == 'A' {
                let first_lhs_ch = content[col - 1][row - 1];
                let last_lhs_ch = content[col + 1][row + 1];

                let lhs = (first_lhs_ch == 'S' && last_lhs_ch == 'M')
                    || (first_lhs_ch == 'M' && last_lhs_ch == 'S');

                //diag top right to bottom left
                let first_rhs_ch = content[col - 1][row + 1];
                let last_rhs_ch = content[col + 1][row - 1];

                let rhs = (first_rhs_ch == 'S' && last_rhs_ch == 'M')
                    || (first_rhs_ch == 'M' && last_rhs_ch == 'S');

                if rhs && lhs {
                    total += 1;
                }
            }
        }
    }
    dbg!(total);
}
