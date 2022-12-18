use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const FACES: [&(i32, i32, i32); 6] = [
    &(1, 0, 0),
    &(-1, 0, 0),
    &(0, 1, 0),
    &(0, -1, 0),
    &(0, 0, 1),
    &(0, 0, -1),
];

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn find_new_steam_points(
    lava_droplets: &HashSet<Coord>,
    steam: &mut HashSet<Coord>,
    previous_steam_points: HashSet<Coord>,
    lower_bound: &Coord,
    upper_bound: &Coord,
) -> HashSet<Coord> {
    let mut new_steam_points: HashSet<Coord> = HashSet::new();
    for p in previous_steam_points {
        for f in FACES {
            let new_p = Coord {
                x: p.x + f.0,
                y: p.y + f.1,
                z: p.z + f.2,
            };
            if new_p.x <= upper_bound.x
                && new_p.y <= upper_bound.y
                && new_p.z <= upper_bound.z
                && new_p.x >= lower_bound.x
                && new_p.y >= lower_bound.y
                && new_p.z >= lower_bound.z
                && !lava_droplets.contains(&new_p)
                && !steam.contains(&new_p)
            {
                steam.insert(new_p.clone());
                new_steam_points.insert(new_p);
            }
        }
    }

    new_steam_points
}

pub(crate) fn day18() {
    let f: File = File::open("data/day18.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut lava_droplets: HashSet<Coord> = HashSet::new();
    let mut min_x: i32 = std::i32::MAX;
    let mut min_y: i32 = std::i32::MAX;
    let mut min_z: i32 = std::i32::MAX;
    let mut max_x: i32 = std::i32::MIN;
    let mut max_y: i32 = std::i32::MIN;
    let mut max_z: i32 = std::i32::MIN;
    for line in input_data {
        let coords = line.split(',').collect::<Vec<&str>>();
        let x = coords.first().unwrap().parse::<i32>().unwrap();
        let y = coords.get(1).unwrap().parse::<i32>().unwrap();
        let z = coords.get(2).unwrap().parse::<i32>().unwrap();
        lava_droplets.insert(Coord { x, y, z });
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if z < min_z {
            min_z = z;
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if z > max_z {
            max_z = z;
        }
    }
    let lower_bound = Coord {
        x: min_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    };
    let upper_bound = Coord {
        x: max_x + 1,
        y: max_y + 1,
        z: max_z + 1,
    };

    let mut steam: HashSet<Coord> = HashSet::new();
    let mut new_steam_points: HashSet<Coord> = HashSet::new();
    new_steam_points.insert(lower_bound.clone());
    while !new_steam_points.is_empty() {
        new_steam_points = find_new_steam_points(
            &lava_droplets,
            &mut steam,
            new_steam_points,
            &lower_bound,
            &upper_bound,
        );
    }

    let mut total_visible_sides: u32 = 0;
    let mut total_visible_external_sides: u32 = 0;
    for p in &lava_droplets {
        for f in FACES {
            let adjacent_p = Coord {
                x: p.x + f.0,
                y: p.y + f.1,
                z: p.z + f.2,
            };
            if !lava_droplets.contains(&adjacent_p) {
                total_visible_sides += 1;
            }
            if steam.contains(&adjacent_p) {
                total_visible_external_sides += 1;
            }
        }
    }
    println!(
        "Surface area of the lava droplets is {}",
        total_visible_sides
    );
    println!(
        "External surface area of the lava droplets is {}",
        total_visible_external_sides
    );
}
