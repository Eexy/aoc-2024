use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn is_in_bound(grid_width: i32, grid_height: i32, coord: &(i32, i32)) -> bool {
    if coord.0 < 0 || coord.0 >= grid_width {
        return false;
    }

    if coord.1 < 0 || coord.1 >= grid_height {
        return false;
    }

    true
}

fn main() -> Result<(), String> {
    let file =
        File::open("input_ori.txt").map_err(|err| format!("failed to open input.txt: {err}"))?;

    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .flatten()
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>();

    let grid_width = lines[0].len() as i32;
    let grid_height = lines.len() as i32;

    println!("grid height: {grid_height}");
    println!("grid width: {grid_width}");

    let mut antennas_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let coord = (x as i32, y as i32);
                antennas_map.entry(c).or_default().push(coord);
            }
        }
    }

    let mut antinnodes: HashSet<(i32, i32)> = HashSet::new();
    for coords in antennas_map.values() {
        for (antenna_idx, antenna_coord) in coords.iter().enumerate() {
            for (next_antenna_idx, next_antenna_coord) in coords.iter().enumerate() {
                if next_antenna_idx == antenna_idx {
                    continue;
                }

                antinnodes.insert(*next_antenna_coord);
                antinnodes.insert(*antenna_coord);

                let move_y = next_antenna_coord.1 - antenna_coord.1;
                let move_x = next_antenna_coord.0 - antenna_coord.0;

                let mut possible_antinode_position =
                    (next_antenna_coord.0 + move_x, next_antenna_coord.1 + move_y);

                while is_in_bound(grid_width, grid_height, &possible_antinode_position) {
                    antinnodes.insert(possible_antinode_position);
                    possible_antinode_position = (
                        possible_antinode_position.0 + move_x,
                        possible_antinode_position.1 + move_y,
                    );
                }
            }
        }
    }

    println!("solution: {}", antinnodes.len());

    Ok(())
}
