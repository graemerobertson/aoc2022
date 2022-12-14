use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DEFAULT_PREFERRED_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

const ADJACENT_LOCATIONS: [Coord; 8] = [
    Coord { x: 1, y: 1 },
    Coord { x: 1, y: 0 },
    Coord { x: 1, y: -1 },
    Coord { x: 0, y: 1 },
    Coord { x: 0, y: -1 },
    Coord { x: -1, y: 1 },
    Coord { x: -1, y: 0 },
    Coord { x: -1, y: -1 },
];

const ADJACENT_LOCATIONS_NORTH: [Coord; 3] = [
    Coord { x: 1, y: -1 },
    Coord { x: 0, y: -1 },
    Coord { x: -1, y: -1 },
];

const ADJACENT_LOCATIONS_SOUTH: [Coord; 3] = [
    Coord { x: 1, y: 1 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 1 },
];

const ADJACENT_LOCATIONS_WEST: [Coord; 3] = [
    Coord { x: -1, y: 1 },
    Coord { x: -1, y: 0 },
    Coord { x: -1, y: -1 },
];

const ADJACENT_LOCATIONS_EAST: [Coord; 3] = [
    Coord { x: 1, y: 1 },
    Coord { x: 1, y: 0 },
    Coord { x: 1, y: -1 },
];

pub(crate) fn day23() {
    let f: File = File::open("data/day23.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut elves: HashSet<Coord> = HashSet::new();
    for (ii, line) in input_data.iter().enumerate() {
        for (jj, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord {
                    x: jj as i32,
                    y: ii as i32,
                });
            }
        }
    }

    let mut preferred_directions = VecDeque::from(DEFAULT_PREFERRED_DIRECTIONS.clone());
    let mut round_count = 0;
    loop {
        round_count += 1;
        let mut proposed_moves_this_round: HashMap<Coord, Vec<Coord>> = HashMap::new();

        for location in &elves {
            let mut elf_should_consider_moving = false;
            for adj_locations in &ADJACENT_LOCATIONS {
                if elves.contains(&Coord {
                    x: location.x + adj_locations.x,
                    y: location.y + adj_locations.y,
                }) {
                    elf_should_consider_moving = true;
                    break;
                }
            }
            if !elf_should_consider_moving {
                continue;
            }

            for direction in &preferred_directions {
                let mut elf_should_move_here = true;
                let current_adj_locations = match direction {
                    Direction::North => ADJACENT_LOCATIONS_NORTH,
                    Direction::South => ADJACENT_LOCATIONS_SOUTH,
                    Direction::West => ADJACENT_LOCATIONS_WEST,
                    Direction::East => ADJACENT_LOCATIONS_EAST,
                };
                for adj_locations in &current_adj_locations {
                    if elves.contains(&Coord {
                        x: location.x + adj_locations.x,
                        y: location.y + adj_locations.y,
                    }) {
                        elf_should_move_here = false;
                        break;
                    }
                }
                if elf_should_move_here {
                    let proposed_new_location = match direction {
                        Direction::North => Coord {
                            x: location.x,
                            y: location.y - 1,
                        },
                        Direction::South => Coord {
                            x: location.x,
                            y: location.y + 1,
                        },
                        Direction::West => Coord {
                            x: location.x - 1,
                            y: location.y,
                        },
                        Direction::East => Coord {
                            x: location.x + 1,
                            y: location.y,
                        },
                    };
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        proposed_moves_this_round.entry(proposed_new_location)
                    {
                        e.insert(vec![*location]);
                    } else {
                        proposed_moves_this_round
                            .get_mut(&proposed_new_location)
                            .unwrap()
                            .push(*location);
                    }
                    break;
                }
            }
        }
        let mut move_has_occurred = false;
        for (new_location, old_locations) in proposed_moves_this_round.iter() {
            // Only one elf wants to move to this new location, so let's make it happen
            if old_locations.len() == 1 {
                move_has_occurred = true;
                elves.remove(old_locations.first().unwrap());
                elves.insert(*new_location);
            }
        }
        if round_count == 10 {
            println!(
                "{} empty ground tiles in the elves rectangle after 10 rounds",
                (elves.iter().map(|c| c.x).max().unwrap() + 1
                    - elves.iter().map(|c| c.x).min().unwrap())
                    * (elves.iter().map(|c| c.y).max().unwrap() + 1
                        - elves.iter().map(|c| c.y).min().unwrap())
                    - elves.len() as i32
            );
        }
        if !move_has_occurred {
            println!("The first round where no elves move is {}", round_count);
            return;
        }
        preferred_directions.rotate_left(1);
    }
}
