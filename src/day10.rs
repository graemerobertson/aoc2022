use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day10() {
    let f: File = File::open("data/day10.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let mut operations: Vec<i32> = vec![];
    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        operations.push(0);
        if unwrapped_line.starts_with("addx") {
            operations.push(
                unwrapped_line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            )
        }
    }

    let mut x: i32 = 1;
    let mut signal_strength = 0;
    let mut screen: Vec<char> = vec!['.'; 240];
    for cycle_number in 1..240 {
        if (cycle_number + 20) % 40 == 0 {
            signal_strength += x * cycle_number;
        }
        if x - 1 == (cycle_number - 1) % 40
            || x + 1 == (cycle_number - 1) % 40
            || x == (cycle_number - 1) % 40
        {
            screen[cycle_number as usize - 1] = '#';
        }
        x += operations.get(cycle_number as usize - 1).unwrap();
    }

    println!("Signal strength is {}", signal_strength);
    println!("Screen:");
    let mut row = "".to_string();
    for (index, c) in screen.iter().enumerate() {
        row.push(*c);
        if (index + 1) % 40 == 0 {
            println!("{}", row);
            row.clear();
        }
    }
}
