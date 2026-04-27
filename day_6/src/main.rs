use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

fn is_out(position: &Position, width: i32, height: i32) -> bool {
    position.x < 0 || position.x >= width || position.y < 0 || position.y >= height
}

fn is_obstacle_up(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> bool {
    obstacles_positions.iter().any(|obstacle_position| {
        obstacle_position.1 == position.y - 1 && position.x == obstacle_position.0
    })
}

fn is_obstacle_down(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> bool {
    obstacles_positions.iter().any(|obstacle_position| {
        obstacle_position.1 == position.y + 1 && obstacle_position.0 == position.x
    })
}

fn is_obstacle_right(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> bool {
    obstacles_positions.iter().any(|obstacle_position| {
        obstacle_position.0 == position.x + 1 && obstacle_position.1 == position.y
    })
}

fn is_obstacle_left(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> bool {
    obstacles_positions.iter().any(|obstacle_position| {
        obstacle_position.0 == position.x - 1 && obstacle_position.1 == position.y
    })
}

fn move_guard(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> Position {
    let is_obstacle_next = match position.dir {
        Direction::Up => is_obstacle_up(position, obstacles_positions),
        Direction::Down => is_obstacle_down(position, obstacles_positions),
        Direction::Left => is_obstacle_left(position, obstacles_positions),
        Direction::Right => is_obstacle_right(position, obstacles_positions),
    };

    if is_obstacle_next {
        return match position.dir {
            Direction::Up => Position {
                x: position.x,
                y: position.y,
                dir: Direction::Right,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y,
                dir: Direction::Left,
            },
            Direction::Right => Position {
                x: position.x,
                y: position.y,
                dir: Direction::Down,
            },
            Direction::Left => Position {
                x: position.x,
                y: position.y,
                dir: Direction::Up,
            },
        };
    }

    match position.dir {
        Direction::Up => Position {
            x: position.x,
            y: position.y - 1,
            dir: Direction::Up,
        },
        Direction::Down => Position {
            x: position.x,
            y: position.y + 1,
            dir: Direction::Down,
        },
        Direction::Right => Position {
            x: position.x + 1,
            y: position.y,
            dir: Direction::Right,
        },
        Direction::Left => Position {
            x: position.x - 1,
            y: position.y,
            dir: Direction::Left,
        },
    }
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
    let mut map = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut current_position = Position {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    map.iter().enumerate().for_each(|(idx, line)| {
        let result = line.iter().position(|&x| x == '^');
        if let Some(first_position) = result {
            current_position.x = first_position as i32;
            current_position.y = idx as i32;
        }
    });

    let mut obstacles_positions = vec![];

    for (y, line) in map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            match ch {
                '^' => {
                    current_position.x = x as i32;
                    current_position.y = y as i32;
                }
                '#' => {
                    obstacles_positions.push((x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((current_position.x, current_position.y));

    let height = map.len() as i32;
    let width = map.last().map_or(0, |line| line.len() as i32);

    // let mut counter = 0;

    while !is_out(&current_position, width, height) {
        // dbg!(&current_position);
        map[current_position.y as usize][current_position.x as usize] = 'X';
        visited.insert((current_position.x, current_position.y));
        current_position = move_guard(&current_position, &obstacles_positions);
        // dbg!(&current_position);
        // counter += 1;
    }

    dbg!(visited.len());
}
