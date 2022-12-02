use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn day02() {
    let f: File = File::open("data/day02.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let mut score = 0;
    for line in reader.lines() {
        let line2 = line.unwrap();
        let mut strategy = line2.chars();
        let they_chose = strategy.next().unwrap();
        let we_chose = strategy.last().unwrap();
        if we_chose == 'X' {
            score += 1;
            if they_chose == 'A' {
                score += 3;
            } else if they_chose == 'B' {
                score += 0;
            } else {
                score += 6;
            }
        } else if we_chose == 'Y' {
            score += 2;
            if they_chose == 'A' {
                score += 6;
            } else if they_chose == 'B' {
                score += 3;
            } else {
                score += 0;
            }
        } else {
            score += 3;
            if they_chose == 'A' {
                score += 0;
            } else if they_chose == 'B' {
                score += 6;
            } else {
                score += 3;
            }
        }
    }
    println!("Part 1: {}", score);

    let f2: File = File::open("data/day02.txt").unwrap();
    let reader2: BufReader<File> = BufReader::new(f2);
    let mut score2 = 0;
    for line in reader2.lines() {
        let line2 = line.unwrap();
        let mut strategy = line2.chars();
        let they_chose = strategy.next().unwrap();
        let we_chose = strategy.last().unwrap();
        if we_chose == 'X' {
            score2 += 0;
            if they_chose == 'A' {
                score2 += 3;
            } else if they_chose == 'B' {
                score2 += 1;
            } else {
                score2 += 2;
            }
        } else if we_chose == 'Y' {
            score2 += 3;
            if they_chose == 'A' {
                score2 += 1;
            } else if they_chose == 'B' {
                score2 += 2;
            } else {
                score2 += 3;
            }
        } else {
            score2 += 6;
            if they_chose == 'A' {
                score2 += 2;
            } else if they_chose == 'B' {
                score2 += 3;
            } else {
                score2 += 1;
            }
        }
    }
    println!("Part 2: {}", score2);
}
