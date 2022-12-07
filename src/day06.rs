use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START_OF_PACKET_MARKER_LENGTH: usize = 4;
const START_OF_MESSAGE_MARKER_LENGTH: usize = 14;

fn get_end_of_unique_seq_index(stream: &str, length: usize) -> Option<usize> {
    (length..stream.len()).find(|&i| {
        stream[i - length..i]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == length
    })
}

pub(crate) fn day06() {
    let f: File = File::open("data/day06.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let stream = reader.lines().next().unwrap().unwrap();
    println!(
        "First start-of-packet marker detected by character {}",
        get_end_of_unique_seq_index(&stream, START_OF_PACKET_MARKER_LENGTH).unwrap()
    );
    println!(
        "First start-of-message marker detected by character {}",
        get_end_of_unique_seq_index(&stream, START_OF_MESSAGE_MARKER_LENGTH).unwrap()
    );
}
