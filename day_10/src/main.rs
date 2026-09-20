use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    val: u32,
    x: usize,
    y: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_next_positions(start_position: &Position, map: &Vec<Vec<u32>>) -> Vec<Position> {
    DIRECTIONS
        .iter()
        .filter_map(|(x, y)| {
            let x = start_position.x.checked_add_signed(*x as isize)?;

            let y = start_position.y.checked_add_signed(*y as isize)?;

            let val = *map.get(y)?.get(x)?;

            (val == start_position.val + 1).then_some(Position { val, x, y })
        })
        .collect()
}

fn find_trail(start_position: Position, map: &Vec<Vec<u32>>) -> i32 {
    let mut next_positions = find_next_positions(&start_position, map);

    let mut sum = 0;
    while let Some(next_position) = next_positions.pop() {
        if next_position.val == 9 {
            sum += 1;
        } else {
            sum += find_trail(next_position, map);
        }
    }

    sum
}

fn main() -> Result<(), String> {
    let file =
        File::open("input.txt").map_err(|err| format!("unable to read input file, {err}"))?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut zeros: Vec<Position> = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if val == &0 {
                zeros.push(Position {
                    val: val.to_owned(),
                    x,
                    y,
                });
            }
        }
    }

    let mut sum = 0;

    while let Some(start) = zeros.pop() {
        sum += find_trail(start, &lines);
    }

    dbg!(sum);

    Ok(())
}
