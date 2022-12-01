use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day01() {
    let f: File = File::open("data/day01.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    // Assume the data contains at least one elf
    let mut number_of_elves: usize = 1;
    let mut calories: Vec<u32> = vec![0; number_of_elves];

    for line in reader.lines() {
        if let Ok(i) = line.unwrap().parse::<u32>() {
            calories[number_of_elves - 1] += i;
        } else {
            // New elf
            number_of_elves += 1;
            calories.push(0);
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
