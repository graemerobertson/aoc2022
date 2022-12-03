use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_priority_value(c: char) -> u32 {
    match c.is_ascii_uppercase() {
        true => c as u32 - 38,
        false => c as u32 - 96,
    }
}

pub(crate) fn day03() {
    let f: File = File::open("data/day03.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let mut rucksacks: Vec<String> = vec![];
    for line in reader.lines() {
        rucksacks.push(line.unwrap());
    }

    println!(
        "Sum of duplicated item priorities is {}",
        rucksacks
            .iter()
            .map(|x| {
                let (first_compartment, second_compartment) = x.split_at(x.len() / 2);
                return get_priority_value(
                    first_compartment
                        .chars()
                        .filter(|c| second_compartment.contains(*c))
                        .last()
                        .unwrap(),
                );
            })
            .sum::<u32>()
    );

    let mut part2: u32 = 0;
    for groups in rucksacks.chunks(3) {
        part2 += get_priority_value(
            groups
                .first()
                .unwrap()
                .chars()
                .filter(|c| {
                    groups.get(1).unwrap().contains(*c) && groups.last().unwrap().contains(*c)
                })
                .last()
                .unwrap(),
        );
    }
    println!("Sum of badge priorities is {}", part2);
}
