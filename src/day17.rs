use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const ROCK_ENTRY_HEIGHT: usize = 4;
const ROCK_ENTRY_X_POSITION: usize = 2;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Rock {
    width: usize,
    coords: Vec<Coord>,
}

pub(crate) fn day17() {
    let rock1: Rock = Rock {
        width: 4,
        coords: vec![
            Coord { x: 0, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 2, y: 0 },
            Coord { x: 3, y: 0 },
        ],
    };
    let rock2: Rock = Rock {
        width: 3,
        coords: vec![
            Coord { x: 1, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 1 },
            Coord { x: 2, y: 1 },
            Coord { x: 1, y: 2 },
        ],
    };
    let rock3: Rock = Rock {
        width: 3,
        coords: vec![
            Coord { x: 0, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ],
    };
    let rock4: Rock = Rock {
        width: 1,
        coords: vec![
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 0, y: 3 },
        ],
    };
    let rock5: Rock = Rock {
        width: 2,
        coords: vec![
            Coord { x: 0, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 1 },
        ],
    };
    let rocks: Vec<Rock> = vec![rock1, rock2, rock3, rock4, rock5];
    let mut current_rock_index: usize = 0;

    let f: File = File::open("data/day17.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let flow: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();
    let flow_len: usize = flow.len();
    let mut current_flow_index: usize = 0;

    let mut cave: Vec<Vec<char>> = vec![vec!['.'; 7000]; 7];
    // Set the floor
    for floor_tile in cave.iter_mut().take(7) {
        floor_tile[0] = '#';
    }

    let mut current_base_level: usize = 0;
    let mut number_of_rocks_landed = 0;
    let mut states: HashMap<String, (usize, usize)> = HashMap::new();
    loop {
        let rock: &Rock = rocks.get(current_rock_index).unwrap();
        let mut rock_position: Coord = Coord {
            x: ROCK_ENTRY_X_POSITION,
            y: ROCK_ENTRY_HEIGHT + current_base_level,
        };
        loop {
            match flow.get(current_flow_index).unwrap() {
                '>' => {
                    if rock_position.x + rock.width < 7 {
                        let mut rock_can_flow = true;
                        for coord in &rock.coords {
                            if cave[coord.x + rock_position.x + 1][coord.y + rock_position.y] != '.'
                            {
                                rock_can_flow = false;
                            }
                        }
                        if rock_can_flow {
                            rock_position.x += 1;
                        }
                    }
                }
                '<' => {
                    if rock_position.x > 0 {
                        let mut rock_can_flow = true;
                        for coord in &rock.coords {
                            if cave[coord.x + rock_position.x - 1][coord.y + rock_position.y] != '.'
                            {
                                rock_can_flow = false;
                            }
                        }
                        if rock_can_flow {
                            rock_position.x -= 1;
                        }
                    }
                }
                _ => {
                    panic!();
                }
            }
            current_flow_index = (current_flow_index + 1) % flow_len;

            let mut rock_can_fall = true;
            for coord in &rock.coords {
                if cave[coord.x + rock_position.x][coord.y + rock_position.y - 1] != '.' {
                    rock_can_fall = false;
                    break;
                }
            }
            if rock_can_fall {
                rock_position.y -= 1;
            } else {
                // Rock settles in current position
                for coord in &rock.coords {
                    cave[coord.x + rock_position.x][coord.y + rock_position.y] = '#';

                    // Set the new current base level
                    if coord.y + rock_position.y > current_base_level {
                        current_base_level = coord.y + rock_position.y;
                    }
                }
                // break this loop and move onto a new rock
                break;
            }
        }

        current_rock_index = (current_rock_index + 1) % 5;
        number_of_rocks_landed += 1;

        // Define our state key as our next flow indicator, our next rock, and the top two
        // rows of the cave that contain rock. That should be sufficient.
        let state_key: String = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            current_rock_index,
            current_flow_index,
            cave[0][current_base_level],
            cave[1][current_base_level],
            cave[2][current_base_level],
            cave[3][current_base_level],
            cave[4][current_base_level],
            cave[5][current_base_level],
            cave[6][current_base_level],
            cave[0][current_base_level - 1],
            cave[1][current_base_level - 1],
            cave[2][current_base_level - 1],
            cave[3][current_base_level - 1],
            cave[4][current_base_level - 1],
            cave[5][current_base_level - 1],
            cave[6][current_base_level - 1],
        );
        // Our state is the current number of rocks and the current height.
        let state: (usize, usize) = (number_of_rocks_landed, current_base_level);
        if let std::collections::hash_map::Entry::Vacant(e) = states.entry(state_key.to_owned()) {
            e.insert(state);
        } else {
            // If we've seen this position before, check if we can extrapolate to 1000000000000 rocks.
            let number_of_rocks_difference =
                number_of_rocks_landed - states.get(&state_key).unwrap().0;
            let height_difference = current_base_level - states.get(&state_key).unwrap().1;
            if (1000000000000 - number_of_rocks_landed) % number_of_rocks_difference == 0 {
                println!(
                    "Height of tower after 1000000000000 rocks is {}",
                    current_base_level
                        + (1000000000000 - number_of_rocks_landed) * height_difference
                            / number_of_rocks_difference
                );
                return;
            }
        }

        if number_of_rocks_landed == 2022 {
            println!("Height of tower after 2022 rocks is {}", current_base_level);
        }
    }
}
