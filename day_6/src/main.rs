use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

fn is_out(position: &Position, width: i32, height: i32) -> bool {
    position.x < 0 || position.x >= width || position.y < 0 || position.y >= height
}

fn is_obstacle_up(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> bool {
    obstacles_positions
        .get(&(position.x, position.y - 1))
        .is_some()
}

fn is_obstacle_down(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> bool {
    obstacles_positions
        .get(&(position.x, position.y + 1))
        .is_some()
}

fn is_obstacle_right(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> bool {
    obstacles_positions
        .get(&(position.x + 1, position.y))
        .is_some()
}

fn is_obstacle_left(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> bool {
    obstacles_positions
        .get(&(position.x - 1, position.y))
        .is_some()
}

fn is_obstacle_next(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> bool {
    match position.dir {
        Direction::Up => is_obstacle_up(position, obstacles_positions),
        Direction::Down => is_obstacle_down(position, obstacles_positions),
        Direction::Left => is_obstacle_left(position, obstacles_positions),
        Direction::Right => is_obstacle_right(position, obstacles_positions),
    }
}

fn move_guard(position: &Position, obstacles_positions: &HashSet<(i32, i32)>) -> Position {
    if is_obstacle_next(position, obstacles_positions) {
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
    let map = reader
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
    let initial_position = current_position.clone();

    let mut obstacles_positions: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            match ch {
                '^' => {
                    current_position.x = x as i32;
                    current_position.y = y as i32;
                }
                '#' => {
                    obstacles_positions.insert((x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    let mut visited: Vec<Position> = vec![];

    let height = map.len() as i32;
    let width = map.last().map_or(0, |line| line.len() as i32);

    while !is_out(&current_position, width, height) {
        let next = move_guard(&current_position, &obstacles_positions);
        visited.push(current_position);
        current_position = next;
    }

    let mut new_obstacle_tested_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut loops = 0;
    for position in visited.iter() {
        let next = move_guard(position, &obstacles_positions);

        let new_obstacle = (next.x, next.y);

        // Check if next position is in the map
        if is_out(&next, width, height) {
            continue;
        };

        // Check if the new obstacle is not at the position of already existing obstacle
        if obstacles_positions.contains(&new_obstacle) {
            continue;
        }

        // Check if the new obstacle is not placed at the initial position
        if new_obstacle == (initial_position.x, initial_position.y) {
            continue;
        }

        // Check that we don't test a new obstacle position twice
        if new_obstacle_tested_positions.contains(&new_obstacle) {
            continue;
        }

        let mut new_position = position.clone();
        let mut temp_positions: HashSet<Position> = HashSet::from([]);

        obstacles_positions.insert(new_obstacle);

        let is_loop = loop {
            if is_out(&new_position, width, height) {
                break false;
            };
            if temp_positions.contains(&new_position) {
                break true;
            };
            let next_position = move_guard(&new_position, &obstacles_positions);
            temp_positions.insert(new_position);
            new_position = next_position;
        };

        if is_loop {
            loops += 1;
        }

        obstacles_positions.remove(&new_obstacle);
        new_obstacle_tested_positions.insert(new_obstacle);
    }

    dbg!(
        visited
            .iter()
            .map(|p| (p.x, p.y))
            .collect::<HashSet<_>>()
            .len()
    );
    dbg!(loops);
}
