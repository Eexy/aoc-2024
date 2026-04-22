use std::{
    fs::File,
    i32,
    io::{BufRead, BufReader},
    process,
};

fn part_1(reader: impl BufRead) {
    let parsed_bytes = reader
        .bytes()
        .filter_map(|byte| Some(byte.ok()? as char))
        .collect::<Vec<_>>();

    let iter = parsed_bytes.iter().enumerate();
    let mut total: i32 = 0;
    let mut is_mul_enable = true;
    for (index, ch) in iter {
        if ch == &'d' {
            let do_slice = &parsed_bytes[index..index + 4].iter().collect::<String>();
            let dont_slice = &parsed_bytes[index..index + 7].iter().collect::<String>();

            if do_slice == "do()" {
                is_mul_enable = true
            } else if dont_slice == "don't()" {
                is_mul_enable = false
            }
        } else if ch == &'m' && is_mul_enable {
            let slice = &parsed_bytes[index..index + 4].iter().collect::<String>();
            if slice == "mul(" {
                let mut lhs = String::from("");
                let mut rhs = String::from("");
                let mut separator_found = false;
                let number_index = index + 4;
                for i in number_index..parsed_bytes.len() {
                    match parsed_bytes[i] {
                        '0'..='9' => {
                            if !separator_found {
                                lhs.push(parsed_bytes[i]);
                            } else {
                                rhs.push(parsed_bytes[i]);
                            }
                        }
                        ')' => {
                            if !separator_found {
                                break;
                            }

                            let lvalue = match lhs.parse::<i32>() {
                                Ok(value) => value,
                                Err(e) => {
                                    eprintln!("{e}");
                                    0
                                }
                            };

                            let rvalue = match rhs.parse::<i32>() {
                                Ok(value) => value,
                                Err(e) => {
                                    eprintln!("{e}");
                                    0
                                }
                            };
                            total += lvalue * rvalue;
                            break;
                        }
                        ',' => separator_found = true,
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
    }

    dbg!(total);
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error reading file {err}");
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    part_1(reader);
}
