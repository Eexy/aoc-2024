use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug, Clone)]
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

fn is_obstacle_next(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> bool {
    match position.dir {
        Direction::Up => is_obstacle_up(position, obstacles_positions),
        Direction::Down => is_obstacle_down(position, obstacles_positions),
        Direction::Left => is_obstacle_left(position, obstacles_positions),
        Direction::Right => is_obstacle_right(position, obstacles_positions),
    }
}

fn move_guard(position: &Position, obstacles_positions: &Vec<(i32, i32)>) -> Position {
    // let is_obstacle_next = match position.dir {
    //     Direction::Up => is_obstacle_up(position, obstacles_positions),
    //     Direction::Down => is_obstacle_down(position, obstacles_positions),
    //     Direction::Left => is_obstacle_left(position, obstacles_positions),
    //     Direction::Right => is_obstacle_right(position, obstacles_positions),
    // };

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

fn is_inital_position(position: &Position, initial_position: &Position) -> bool {
    position.x == initial_position.x && position.y == initial_position.y
}

fn create_new_obstacle(
    position: &Position,
    initial_position: &Position,
    width: i32,
    height: i32,
) -> Option<(i32, i32)> {
    if is_inital_position(position, initial_position) {
        return None;
    }

    match position.dir {
        Direction::Up => {
            if position.y - 1 < 0 {
                return None;
            }

            Some((position.x, position.y - 1))
        }
        Direction::Down => {
            if position.y + 1 == height {
                return None;
            }
            Some((position.x, position.y + 1))
        }
        Direction::Left => {
            if position.x - 1 < 0 {
                return None;
            }

            Some((position.x - 1, position.y))
        }

        Direction::Right => {
            if position.x + 1 == width {
                return None;
            }

            Some((position.x + 1, position.y))
        }
    }
}

fn is_next_position_new_obstacle(position: &Position, new_obstacle: &(i32, i32)) -> bool {
    match position.dir {
        Direction::Up => position.y - 1 == new_obstacle.1 && position.x == new_obstacle.0,
        Direction::Down => position.y + 1 == new_obstacle.1 && position.x == new_obstacle.0,
        Direction::Left => position.y == new_obstacle.1 && position.x - 1 == new_obstacle.0,
        Direction::Right => position.y == new_obstacle.1 && position.x + 1 == new_obstacle.0,
    }
}

fn print_position(direction: &Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

fn print_map(map: &Vec<Vec<char>>, new_obstacle: &(i32, i32), position: &Position) {
    for (y, line) in map.iter().enumerate() {
        println!(
            "{}",
            line.iter()
                .enumerate()
                .map(|(idx, ch)| {
                    if idx as i32 == new_obstacle.0 && y as i32 == new_obstacle.1 {
                        return '0';
                    } else if idx as i32 == position.x && y as i32 == position.y {
                        return print_position(&position.dir);
                    }

                    *ch
                })
                .collect::<String>()
        );
    }

    println!("");
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
    let initial_position = current_position.clone();

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
    let mut already_tested_new_positions: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((current_position.x, current_position.y));

    let height = map.len() as i32;
    let width = map.last().map_or(0, |line| line.len() as i32);
    let mut loops = 0;

    while !is_out(&current_position, width, height) {
        let possible_new_obstacle =
            create_new_obstacle(&current_position, &initial_position, width, height);

        if let Some(new_obstacle) = possible_new_obstacle {
            // print_map(&map, &new_obstacle, &current_position);
            let mut visited_positions: HashMap<(i32, i32, Direction), i32> = HashMap::new();

            let mut new_position = current_position.clone();
            visited_positions
                .entry((new_position.x, new_position.y, new_position.dir.clone()))
                .and_modify(|v| *v += 1)
                .or_default();
            let mut new_obstacles_positions = obstacles_positions.clone();
            new_obstacles_positions.push(new_obstacle);

            while !is_out(&new_position, width, height)
                && visited_positions
                    .get(&(new_position.x, new_position.y, new_position.dir.clone()))
                    .map_or(true, |value| *value < 2)
                && !already_tested_new_positions.contains(&(new_obstacle.0, new_obstacle.1))
            {
                // if is_next_position_new_obstacle(&new_position, &new_obstacle) {
                //     counter_visited_new_obstacle += 1;
                // }
                new_position = move_guard(&new_position, &new_obstacles_positions);

                visited_positions
                    .entry((new_position.x, new_position.y, new_position.dir.clone()))
                    .and_modify(|v| *v += 1)
                    .or_default();

                if visited_positions
                    .get(&(new_position.x, new_position.y, new_position.dir.clone()))
                    .is_some_and(|v| *v >= 2)
                {
                    loops += 1;
                }
            }

            already_tested_new_positions.insert((new_obstacle.0, new_obstacle.1));

            // if counter_visited_new_obstacle >= 4 {
            //     loops += 1;
            // }
        }

        // map[current_position.y as usize][current_position.x as usize] = 'X';
        visited.insert((current_position.x, current_position.y));
        dbg!(visited.len());
        current_position = move_guard(&current_position, &obstacles_positions);
        // dbg!(&counter);
        // dbg!(&current_position);
        // counter += 1;
    }

    dbg!(visited.len());
    dbg!(loops);
}
