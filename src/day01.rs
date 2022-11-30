use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut depths: Vec<u32> = vec![];
    for line in reader.lines() {
        depths.push(line.unwrap().parse::<u32>().unwrap());
    }
}
