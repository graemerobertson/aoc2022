use std::fs::File;
use std::io::{self, BufRead, BufReader};

const MAXIMUM_CONCEIVABLE_SIZE_OF_CAVE: usize = 1000;
const SAND_ENTRY_X_COORD: usize = 500;

fn drop_sand(grid: &mut [Vec<bool>]) -> bool {
    let mut x_coord = SAND_ENTRY_X_COORD;
    for i in 0..MAXIMUM_CONCEIVABLE_SIZE_OF_CAVE - 1 {
        if grid[x_coord][i + 1] && grid[x_coord - 1][i + 1] && grid[x_coord + 1][i + 1] {
            grid[x_coord][i] = true;
            return true;
        } else if grid[x_coord][i + 1] && grid[x_coord - 1][i + 1] {
            x_coord += 1;
        } else if grid[x_coord][i + 1] {
            x_coord -= 1;
        }
    }
    false
}

pub(crate) fn day14() {
    let f: File = File::open("data/day14.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut part1_cave: Vec<Vec<bool>> =
        vec![vec![false; MAXIMUM_CONCEIVABLE_SIZE_OF_CAVE]; MAXIMUM_CONCEIVABLE_SIZE_OF_CAVE];
    let mut max_cave_height = 0;
    for line in input_data {
        let coords = line.split(" -> ").collect::<Vec<&str>>();
        for entry in 0..coords.len() - 1 {
            let mut coords1 = coords.get(entry).unwrap().split(',');
            let x1 = coords1.next().unwrap().parse::<usize>().unwrap();
            let y1 = coords1.next().unwrap().parse::<usize>().unwrap();
            let mut coords2 = coords.get(entry + 1).unwrap().split(',');
            let x2 = coords2.next().unwrap().parse::<usize>().unwrap();
            let y2 = coords2.next().unwrap().parse::<usize>().unwrap();
            if x1 == x2 {
                for y in y1..y2 + 1 {
                    part1_cave[x1][y] = true;
                }
                for y in y2..y1 + 1 {
                    part1_cave[x1][y] = true;
                }
            } else {
                for row in part1_cave.iter_mut().take(x2 + 1).skip(x1) {
                    row[y1] = true;
                }
                for row in part1_cave.iter_mut().take(x1 + 1).skip(x2) {
                    row[y1] = true;
                }
            }
            if y1 > max_cave_height {
                max_cave_height = y1;
            }
        }
    }

    let mut part2_cave = part1_cave.clone();
    for row in part2_cave.iter_mut().take(MAXIMUM_CONCEIVABLE_SIZE_OF_CAVE) {
        row[max_cave_height + 2] = true;
    }

    let mut count_of_sand_units = 0;
    loop {
        if !drop_sand(&mut part1_cave) {
            println!("{} units of sand come to rest", count_of_sand_units);
            break;
        }
        count_of_sand_units += 1;
    }

    count_of_sand_units = 0;
    loop {
        count_of_sand_units += 1;
        drop_sand(&mut part2_cave);
        if part2_cave[SAND_ENTRY_X_COORD][0] {
            println!(
                "{} units of sand come to rest when there's a floor",
                count_of_sand_units
            );
            break;
        }
    }
}
