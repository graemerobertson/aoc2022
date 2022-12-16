use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const SIZE: i128 = 4000001;

struct Sensor {
    sensor_x: i128,
    sensor_y: i128,
    manhattan_distance: u128,
}

struct XRange {
    min: i128,
    max: i128,
}

fn manhattan_distance(x1: i128, y1: i128, x2: i128, y2: i128) -> u128 {
    (x1 - x2).unsigned_abs() + (y1 - y2).unsigned_abs()
}

pub(crate) fn day15() {
    let f: File = File::open("data/day15.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut sensors: Vec<Sensor> = vec![];
    for line in input_data {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)$"
            )
            .unwrap();
        }
        let cap = RE.captures(&line).unwrap();
        let sensor_x = cap[1].parse::<i128>().unwrap();
        let sensor_y = cap[2].parse::<i128>().unwrap();
        let beacon_x = cap[3].parse::<i128>().unwrap();
        let beacon_y = cap[4].parse::<i128>().unwrap();
        sensors.push(Sensor {
            sensor_x,
            sensor_y,
            manhattan_distance: manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y),
        });
    }

    for y in 0..SIZE {
        let mut xranges: Vec<XRange> = vec![];
        for s in &sensors {
            let x_reach_on_this_y = s.manhattan_distance as i128 - i128::abs(y - s.sensor_y);
            if x_reach_on_this_y > 0 {
                xranges.push(XRange {
                    min: std::cmp::max(0, s.sensor_x - x_reach_on_this_y),
                    max: std::cmp::min(SIZE, s.sensor_x + x_reach_on_this_y),
                });
            }
        }
        xranges.sort_by(|a, b| a.min.cmp(&b.min));
        let mut tracker = -1;
        for range in &xranges {
            if range.min > tracker + 1 {
                println!(
                    "Tuning frequency of the distress beacon is {}",
                    (tracker + 1) * 4000000 + y
                );
                return;
            } else if tracker < range.max {
                tracker = range.max;
            }
        }
    }
}
