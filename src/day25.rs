use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day25() {
    let f: File = File::open("data/day25.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut snafu_sum_vec: Vec<i32> = vec![];
    for line in input_data {
        for (index, c) in line.chars().rev().enumerate() {
            if index == snafu_sum_vec.len() {
                snafu_sum_vec.push(0);
            }
            match c {
                '=' => snafu_sum_vec[index] -= 2,
                '-' => snafu_sum_vec[index] -= 1,
                '0' => (),
                '1' => snafu_sum_vec[index] += 1,
                '2' => snafu_sum_vec[index] += 2,
                _ => panic!(),
            }
        }
    }

    let mut carryover = 0;
    let mut answer = "".to_string();
    for snafu_digit in &mut snafu_sum_vec {
        *snafu_digit += carryover;
        carryover = 0;
        while *snafu_digit > 2 {
            carryover += 1;
            *snafu_digit -= 5;
        }
        while *snafu_digit < -2 {
            carryover -= 1;
            *snafu_digit += 5;
        }
        match snafu_digit {
            -2 => answer.push('='),
            -1 => answer.push('-'),
            0 => answer.push('0'),
            1 => answer.push('1'),
            2 => answer.push('2'),
            _ => panic!(),
        }
    }
    println!(
        "SNAFU number for Bob's console is {}",
        answer.chars().rev().collect::<String>()
    );
}
