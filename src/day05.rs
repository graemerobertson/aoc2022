use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Instruction {
    number_of_crates_to_move: u32,
    src_stack: usize,
    dst_stack: usize,
}

fn operate_cratemover_9000(list_of_instructions: &Vec<Instruction>, stacks: &mut [Vec<char>]) {
    for instruction in list_of_instructions {
        for _ in 0..instruction.number_of_crates_to_move {
            let crate_to_move = stacks
                .get_mut(instruction.src_stack)
                .unwrap()
                .pop()
                .unwrap();
            stacks
                .get_mut(instruction.dst_stack)
                .unwrap()
                .push(crate_to_move);
        }
    }
}

fn operate_cratemover_9001(list_of_instructions: &Vec<Instruction>, stacks: &mut [Vec<char>]) {
    for instruction in list_of_instructions {
        let mut crates_to_move: Vec<char> = vec![];
        for _ in 0..instruction.number_of_crates_to_move {
            crates_to_move.push(
                stacks
                    .get_mut(instruction.src_stack)
                    .unwrap()
                    .pop()
                    .unwrap(),
            );
        }
        crates_to_move.reverse();
        stacks
            .get_mut(instruction.dst_stack)
            .unwrap()
            .append(&mut crates_to_move);
    }
}

pub(crate) fn day05() {
    // I can't be bothered to parse this input.
    let initial_stacks: Vec<Vec<char>> = vec![
        vec!['V', 'C', 'D', 'R', 'Z', 'G', 'B', 'W'],
        vec!['G', 'W', 'F', 'C', 'B', 'S', 'T', 'V'],
        vec!['C', 'B', 'S', 'N', 'W'],
        vec!['Q', 'G', 'M', 'N', 'J', 'V', 'C', 'P'],
        vec!['T', 'S', 'L', 'F', 'D', 'H', 'B'],
        vec!['J', 'V', 'T', 'W', 'M', 'N'],
        vec!['P', 'F', 'L', 'C', 'S', 'T', 'G'],
        vec!['B', 'D', 'Z'],
        vec!['M', 'N', 'Z', 'W'],
    ];

    let f: File = File::open("data/day05.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let mut list_of_instructions: Vec<Instruction> = vec![];
    for line in reader.lines() {
        let instruction_text = line.unwrap();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let cap = RE.captures(&instruction_text).unwrap();
        let instruction: Instruction = Instruction {
            number_of_crates_to_move: cap[1].parse::<u32>().unwrap(),
            src_stack: cap[2].parse::<usize>().unwrap() - 1,
            dst_stack: cap[3].parse::<usize>().unwrap() - 1,
        };
        list_of_instructions.push(instruction);
    }

    let mut cratemover_9000_stacks = initial_stacks.clone();
    operate_cratemover_9000(&list_of_instructions, &mut cratemover_9000_stacks);
    println!(
        "{}",
        cratemover_9000_stacks
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    );

    let mut cratemover_9001_stacks = initial_stacks;
    operate_cratemover_9001(&list_of_instructions, &mut cratemover_9001_stacks);
    println!(
        "{}",
        cratemover_9001_stacks
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    );
}
