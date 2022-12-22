use std::fs::File;
use std::io::{self, BufRead, BufReader};

const WIDTH: usize = 150;
const HEIGHT: usize = 200;

struct Instruction {
    rotation: Option<Rotation>,
    movement: Option<usize>,
}

#[derive(PartialEq, Copy, Clone)]
enum Rotation {
    Right,
    Left,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Terrain {
    Path,
    Wall,
    Blank,
}

#[derive(PartialEq, Debug, Clone)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn rotate_right(&self) -> Facing {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn rotate_left(&self) -> Facing {
        match self {
            Facing::Right => Facing::Up,
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
        }
    }
}

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn part1(
    instructions: &Vec<Instruction>,
    grid: &[Vec<Terrain>],
    initial_facing: Facing,
    initial_location: Coord,
) -> usize {
    let mut current_location = initial_location;
    let mut current_facing = initial_facing;
    // Follow instructions
    for instruction in instructions {
        if instruction.movement.is_some() {
            if current_facing == Facing::Right {
                let mut new_x = current_location.x;
                for _ in 0..instruction.movement.unwrap() {
                    if grid[(new_x + 1) % WIDTH][current_location.y] == Terrain::Wall {
                        break;
                    } else if grid[(new_x + 1) % WIDTH][current_location.y] == Terrain::Path {
                        new_x = (new_x + 1) % WIDTH;
                    } else {
                        let mut delta = 1;
                        while grid[(new_x + 1 + delta) % WIDTH][current_location.y]
                            == Terrain::Blank
                        {
                            delta += 1;
                        }
                        if grid[(new_x + 1 + delta) % WIDTH][current_location.y] == Terrain::Wall {
                            break;
                        } else if grid[(new_x + 1 + delta) % WIDTH][current_location.y]
                            == Terrain::Path
                        {
                            new_x = (new_x + 1 + delta) % WIDTH;
                        }
                    }
                }
                current_location = Coord {
                    x: new_x,
                    y: current_location.y,
                };
            } else if current_facing == Facing::Left {
                let mut new_x = current_location.x;
                for _ in 0..instruction.movement.unwrap() {
                    if grid[(WIDTH + new_x - 1) % WIDTH][current_location.y] == Terrain::Wall {
                        break;
                    } else if grid[(WIDTH + new_x - 1) % WIDTH][current_location.y] == Terrain::Path
                    {
                        new_x = (WIDTH + new_x - 1) % WIDTH;
                    } else {
                        let mut delta = 1;
                        while grid[(WIDTH + new_x - 1 - delta) % WIDTH][current_location.y]
                            == Terrain::Blank
                        {
                            delta += 1;
                        }
                        if grid[(WIDTH + new_x - 1 - delta) % WIDTH][current_location.y]
                            == Terrain::Wall
                        {
                            break;
                        } else if grid[(WIDTH + new_x - 1 - delta) % WIDTH][current_location.y]
                            == Terrain::Path
                        {
                            new_x = (WIDTH + new_x - 1 - delta) % WIDTH;
                        }
                    }
                }
                current_location = Coord {
                    x: new_x,
                    y: current_location.y,
                };
            } else if current_facing == Facing::Down {
                let mut new_y = current_location.y;
                for _ in 0..instruction.movement.unwrap() {
                    if grid[current_location.x][(new_y + 1) % HEIGHT] == Terrain::Wall {
                        break;
                    } else if grid[current_location.x][(new_y + 1) % HEIGHT] == Terrain::Path {
                        new_y = (new_y + 1) % HEIGHT;
                    } else {
                        let mut delta = 1;
                        while grid[current_location.x][(new_y + 1 + delta) % HEIGHT]
                            == Terrain::Blank
                        {
                            delta += 1;
                        }
                        if grid[current_location.x][(new_y + 1 + delta) % HEIGHT] == Terrain::Wall {
                            break;
                        } else if grid[current_location.x][(new_y + 1 + delta) % HEIGHT]
                            == Terrain::Path
                        {
                            new_y = (new_y + 1 + delta) % HEIGHT;
                        }
                    }
                }
                current_location = Coord {
                    x: current_location.x,
                    y: new_y,
                };
            } else if current_facing == Facing::Up {
                let mut new_y = current_location.y;
                for _ in 0..instruction.movement.unwrap() {
                    if grid[current_location.x][(HEIGHT + new_y - 1) % HEIGHT] == Terrain::Wall {
                        break;
                    } else if grid[current_location.x][(HEIGHT + new_y - 1) % HEIGHT]
                        == Terrain::Path
                    {
                        new_y = (HEIGHT + new_y - 1) % HEIGHT;
                    } else {
                        let mut delta = 1;
                        while grid[current_location.x][(HEIGHT + new_y - 1 - delta) % HEIGHT]
                            == Terrain::Blank
                        {
                            delta += 1;
                        }
                        if grid[current_location.x][(HEIGHT + new_y - 1 - delta) % HEIGHT]
                            == Terrain::Wall
                        {
                            break;
                        } else if grid[current_location.x][(HEIGHT + new_y - 1 - delta) % HEIGHT]
                            == Terrain::Path
                        {
                            new_y = (HEIGHT + new_y - 1 - delta) % HEIGHT;
                        }
                    }
                }
                current_location = Coord {
                    x: current_location.x,
                    y: new_y,
                };
            }
        } else {
            match instruction.rotation.unwrap() {
                Rotation::Right => current_facing = current_facing.rotate_right(),
                Rotation::Left => current_facing = current_facing.rotate_left(),
            }
        }
    }
    (current_location.x + 1) * 4 + (current_location.y + 1) * 1000 + current_facing as usize
}

fn move_right_3d(
    grid: &[Vec<Terrain>],
    distance: usize,
    initial_location: Coord,
) -> (Coord, Facing) {
    let mut new_x = initial_location.x;
    for ii in 1..distance + 1 {
        if grid[(new_x + 1) % WIDTH][initial_location.y] == Terrain::Wall {
            break;
        } else if grid[(new_x + 1) % WIDTH][initial_location.y] == Terrain::Path {
            new_x = (new_x + 1) % WIDTH;
        } else if initial_location.y <= 49 {
            let new_location: Coord = Coord {
                x: 99,
                y: 149 - initial_location.y,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_left_3d(grid, distance - ii, new_location);
            }
        } else if 50 <= initial_location.y && initial_location.y <= 99 {
            let new_location: Coord = Coord {
                x: initial_location.y + 50,
                y: 49,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_up_3d(grid, distance - ii, new_location);
            }
        } else if 100 <= initial_location.y && initial_location.y <= 149 {
            let new_location: Coord = Coord {
                x: 149,
                y: 149 - initial_location.y,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_left_3d(grid, distance - ii, new_location);
            }
        } else if 150 <= initial_location.y && initial_location.y <= 199 {
            let new_location: Coord = Coord {
                x: initial_location.y - 100,
                y: 149,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_up_3d(grid, distance - ii, new_location);
            }
        } else {
            panic!();
        }
    }
    (
        Coord {
            x: new_x,
            y: initial_location.y,
        },
        Facing::Right,
    )
}

fn move_left_3d(
    grid: &[Vec<Terrain>],
    distance: usize,
    initial_location: Coord,
) -> (Coord, Facing) {
    let mut new_x = initial_location.x;
    for ii in 1..distance + 1 {
        if grid[(WIDTH + new_x - 1) % WIDTH][initial_location.y] == Terrain::Wall {
            break;
        } else if grid[(WIDTH + new_x - 1) % WIDTH][initial_location.y] == Terrain::Path {
            new_x = (WIDTH + new_x - 1) % WIDTH;
        } else if initial_location.y <= 49 {
            let new_location: Coord = Coord {
                x: 0,
                y: 149 - initial_location.y,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_right_3d(grid, distance - ii, new_location);
            }
        } else if 50 <= initial_location.y && initial_location.y <= 99 {
            let new_location: Coord = Coord {
                x: initial_location.y - 50,
                y: 100,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_down_3d(grid, distance - ii, new_location);
            }
        } else if 100 <= initial_location.y && initial_location.y <= 149 {
            let new_location: Coord = Coord {
                x: 50,
                y: 149 - initial_location.y,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_right_3d(grid, distance - ii, new_location);
            }
        } else if 150 <= initial_location.y && initial_location.y <= 199 {
            let new_location: Coord = Coord {
                x: initial_location.y - 100,
                y: 0,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_down_3d(grid, distance - ii, new_location);
            }
        } else {
            panic!();
        }
    }
    (
        Coord {
            x: new_x,
            y: initial_location.y,
        },
        Facing::Left,
    )
}

fn move_down_3d(
    grid: &[Vec<Terrain>],
    distance: usize,
    initial_location: Coord,
) -> (Coord, Facing) {
    let mut new_y = initial_location.y;
    for ii in 1..distance + 1 {
        if grid[initial_location.x][(new_y + 1) % HEIGHT] == Terrain::Wall {
            break;
        } else if grid[initial_location.x][(new_y + 1) % HEIGHT] == Terrain::Path {
            new_y = (new_y + 1) % HEIGHT;
        } else if initial_location.x <= 49 {
            let new_location: Coord = Coord {
                x: initial_location.x + 100,
                y: 0,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_down_3d(grid, distance - ii, new_location);
            }
        } else if 50 <= initial_location.x && initial_location.x <= 99 {
            let new_location: Coord = Coord {
                x: 49,
                y: initial_location.x + 100,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_left_3d(grid, distance - ii, new_location);
            }
        } else if 100 <= initial_location.x && initial_location.x <= 149 {
            let new_location: Coord = Coord {
                x: 99,
                y: initial_location.x - 50,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_left_3d(grid, distance - ii, new_location);
            }
        } else {
            panic!();
        }
    }
    (
        Coord {
            x: initial_location.x,
            y: new_y,
        },
        Facing::Down,
    )
}

fn move_up_3d(grid: &[Vec<Terrain>], distance: usize, initial_location: Coord) -> (Coord, Facing) {
    let mut new_y = initial_location.y;
    for ii in 1..distance + 1 {
        if grid[initial_location.x][(HEIGHT + new_y - 1) % HEIGHT] == Terrain::Wall {
            break;
        } else if grid[initial_location.x][(HEIGHT + new_y - 1) % HEIGHT] == Terrain::Path {
            new_y = (HEIGHT + new_y - 1) % HEIGHT;
        } else if initial_location.x <= 49 {
            let new_location: Coord = Coord {
                x: 50,
                y: initial_location.x + 50,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_right_3d(grid, distance - ii, new_location);
            }
        } else if 50 <= initial_location.x && initial_location.x <= 99 {
            let new_location: Coord = Coord {
                x: 0,
                y: initial_location.x + 100,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_right_3d(grid, distance - ii, new_location);
            }
        } else if 100 <= initial_location.x && initial_location.x <= 149 {
            let new_location: Coord = Coord {
                x: initial_location.x - 100,
                y: 199,
            };
            if grid[new_location.x][new_location.y] == Terrain::Wall {
                break;
            } else {
                return move_up_3d(grid, distance - ii, new_location);
            }
        } else {
            panic!();
        }
    }
    (
        Coord {
            x: initial_location.x,
            y: new_y,
        },
        Facing::Up,
    )
}

fn part2(
    instructions: &Vec<Instruction>,
    grid: &[Vec<Terrain>],
    initial_facing: Facing,
    initial_location: Coord,
) -> usize {
    let mut current_location = initial_location;
    let mut current_facing = initial_facing;
    // Follow instructions
    for instruction in instructions {
        if instruction.movement.is_some() {
            if current_facing == Facing::Right {
                (current_location, current_facing) = move_right_3d(
                    grid,
                    instruction.movement.unwrap(),
                    current_location.clone(),
                );
            } else if current_facing == Facing::Left {
                (current_location, current_facing) = move_left_3d(
                    grid,
                    instruction.movement.unwrap(),
                    current_location.clone(),
                );
            } else if current_facing == Facing::Down {
                (current_location, current_facing) = move_down_3d(
                    grid,
                    instruction.movement.unwrap(),
                    current_location.clone(),
                );
            } else if current_facing == Facing::Up {
                (current_location, current_facing) = move_up_3d(
                    grid,
                    instruction.movement.unwrap(),
                    current_location.clone(),
                );
            }
        } else {
            match instruction.rotation.unwrap() {
                Rotation::Right => current_facing = current_facing.rotate_right(),
                Rotation::Left => current_facing = current_facing.rotate_left(),
            }
        }
    }
    (current_location.x + 1) * 4 + (current_location.y + 1) * 1000 + current_facing as usize
}

pub(crate) fn day22() {
    let f: File = File::open("data/day22.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    // Parse instructions
    let instructions_str: String = input_data.pop().unwrap();
    let mut instructions: Vec<Instruction> = vec![];
    let mut current_number = "".to_string();
    for c in instructions_str.chars() {
        if c.is_numeric() {
            current_number.push(c);
        } else {
            instructions.push(Instruction {
                rotation: None,
                movement: Some(current_number.parse::<usize>().unwrap()),
            });
            current_number.clear();
            let rotation = match c {
                'R' => Some(Rotation::Right),
                'L' => Some(Rotation::Left),
                _ => panic!(),
            };
            instructions.push(Instruction {
                rotation,
                movement: None,
            });
        }
    }
    if !current_number.is_empty() {
        instructions.push(Instruction {
            rotation: None,
            movement: Some(current_number.parse::<usize>().unwrap()),
        });
    }

    // Remove blank line
    input_data.pop();

    // Parse grid
    let mut grid: Vec<Vec<Terrain>> = vec![vec![Terrain::Blank; HEIGHT]; WIDTH];
    for (ii, line) in input_data.iter().enumerate() {
        for (jj, c) in line.chars().enumerate() {
            match c {
                '#' => grid[jj][ii] = Terrain::Wall,
                '.' => grid[jj][ii] = Terrain::Path,
                _ => (),
            }
        }
    }

    // Set initial state
    let initial_facing: Facing = Facing::Right;
    let initial_location: Coord = Coord {
        x: grid
            .iter()
            .position(|x| x.first().unwrap() == &Terrain::Path)
            .unwrap(),
        y: 0,
    };

    println!(
        "Part 1 final password is {}",
        part1(
            &instructions,
            &grid.clone(),
            initial_facing.clone(),
            initial_location.clone()
        )
    );
    println!(
        "Part 2 final password is {}",
        part2(
            &instructions,
            &grid.clone(),
            initial_facing,
            initial_location
        )
    );
}
