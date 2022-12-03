use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    // Assume the data contains at least one elf
    let mut calories: Vec<u32> = vec![];
    let mut current_calories: u32 = 0;

    for line in reader.lines() {
        if let Ok(i) = line.unwrap().parse::<u32>() {
            current_calories += i;
        } else {
            // New elf
            calories.push(current_calories);
            current_calories = 0;
        }
    }

    calories.sort();
    println!(
        "Elf with the most calories is carrying {} calories",
        calories.last().unwrap()
    );
    println!(
        "Three elves with the most calories are carrying {} calories",
        calories[calories.len() - 3..calories.len()]
            .iter()
            .sum::<u32>()
    );
}
