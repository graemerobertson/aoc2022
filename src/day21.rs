use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct UnsolvedMonkey {
    name: String,
    monkey_name1: String,
    monkey_name2: String,
    op: char,
}

fn find_root(
    solved_monkeys: &mut HashMap<String, i128>,
    unsolved_monkeys: &Vec<UnsolvedMonkey>,
) -> i128 {
    while !solved_monkeys.contains_key("root") {
        for monkey in unsolved_monkeys {
            if !solved_monkeys.contains_key(&monkey.name)
                && solved_monkeys.contains_key(&monkey.monkey_name1)
                && solved_monkeys.contains_key(&monkey.monkey_name2)
            {
                let monkey_number1 = solved_monkeys.get(&monkey.monkey_name1).unwrap();
                let monkey_number2 = solved_monkeys.get(&monkey.monkey_name2).unwrap();
                let new_number = match monkey.op {
                    '+' => monkey_number1 + monkey_number2,
                    '-' => monkey_number1 - monkey_number2,
                    '*' => monkey_number1 * monkey_number2,
                    '/' => monkey_number1 / monkey_number2,
                    _ => panic!(),
                };
                solved_monkeys.insert(monkey.name.clone(), new_number);
            }
        }
    }
    *solved_monkeys.get("root").unwrap()
}

fn find_humn(
    solved_monkeys: &mut HashMap<String, i128>,
    unsolved_monkeys: &Vec<UnsolvedMonkey>,
) -> i128 {
    solved_monkeys.remove("humn");
    while !solved_monkeys.contains_key("humn") {
        for monkey in unsolved_monkeys {
            if !solved_monkeys.contains_key(&monkey.name)
                && solved_monkeys.contains_key(&monkey.monkey_name1)
                && solved_monkeys.contains_key(&monkey.monkey_name2)
            {
                let monkey_number1 = solved_monkeys.get(&monkey.monkey_name1).unwrap();
                let monkey_number2 = solved_monkeys.get(&monkey.monkey_name2).unwrap();
                let new_number = match monkey.op {
                    '+' => monkey_number1 + monkey_number2,
                    '-' => monkey_number1 - monkey_number2,
                    '*' => monkey_number1 * monkey_number2,
                    '/' => monkey_number1 / monkey_number2,
                    _ => panic!(),
                };
                solved_monkeys.insert(monkey.name.clone(), new_number);
            } else if solved_monkeys.contains_key(&monkey.name)
                && !solved_monkeys.contains_key(&monkey.monkey_name1)
                && solved_monkeys.contains_key(&monkey.monkey_name2)
            {
                let monkey_number1 = solved_monkeys.get(&monkey.name).unwrap();
                let monkey_number2 = solved_monkeys.get(&monkey.monkey_name2).unwrap();
                let new_number = match monkey.op {
                    '+' => monkey_number1 - monkey_number2,
                    '-' => monkey_number1 + monkey_number2,
                    '*' => monkey_number1 / monkey_number2,
                    '/' => monkey_number1 * monkey_number2,
                    _ => panic!(),
                };
                solved_monkeys.insert(monkey.monkey_name1.clone(), new_number);
            } else if solved_monkeys.contains_key(&monkey.name)
                && solved_monkeys.contains_key(&monkey.monkey_name1)
                && !solved_monkeys.contains_key(&monkey.monkey_name2)
            {
                let monkey_number1 = solved_monkeys.get(&monkey.name).unwrap();
                let monkey_number2 = solved_monkeys.get(&monkey.monkey_name1).unwrap();
                let new_number = match monkey.op {
                    '+' => monkey_number1 - monkey_number2,
                    '-' => monkey_number2 - monkey_number1,
                    '*' => monkey_number1 / monkey_number2,
                    '/' => monkey_number2 / monkey_number1,
                    _ => panic!(),
                };
                solved_monkeys.insert(monkey.monkey_name2.clone(), new_number);
            } else if monkey.name == "root" {
                if solved_monkeys.contains_key(&monkey.monkey_name1) {
                    solved_monkeys.insert(
                        monkey.monkey_name2.clone(),
                        *solved_monkeys.get(&monkey.monkey_name1).unwrap(),
                    );
                } else if solved_monkeys.contains_key(&monkey.monkey_name2) {
                    solved_monkeys.insert(
                        monkey.monkey_name1.clone(),
                        *solved_monkeys.get(&monkey.monkey_name2).unwrap(),
                    );
                }
            }
        }
    }
    *solved_monkeys.get("humn").unwrap()
}

pub(crate) fn day21() {
    let f: File = File::open("data/day21.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut solved_monkeys: HashMap<String, i128> = HashMap::new();
    let mut unsolved_monkeys: Vec<UnsolvedMonkey> = vec![];
    for line in input_data {
        lazy_static! {
            static ref RE_SOLVED_MONKEY: Regex = Regex::new(r"^(.*): (-*\d+)$").unwrap();
        }
        lazy_static! {
            static ref RE_UNSOLVED_MONKEY: Regex =
                Regex::new(r"^([a-z]+): ([a-z]+) ([+-/*]) ([a-z]+)$").unwrap();
        }
        if RE_SOLVED_MONKEY.captures(&line).is_some() {
            let cap = RE_SOLVED_MONKEY.captures(&line).unwrap();
            solved_monkeys.insert(cap[1].parse().unwrap(), cap[2].parse::<i128>().unwrap());
        } else {
            let cap = RE_UNSOLVED_MONKEY.captures(&line).unwrap();
            unsolved_monkeys.push(UnsolvedMonkey {
                name: cap[1].parse().unwrap(),
                monkey_name1: cap[2].parse().unwrap(),
                monkey_name2: cap[4].parse().unwrap(),
                op: cap[3].parse::<char>().unwrap(),
            });
        }
    }
    println!(
        "root's number is: {}",
        find_root(&mut solved_monkeys.clone(), &unsolved_monkeys)
    );
    println!(
        "humn's number is: {}",
        find_humn(&mut solved_monkeys.clone(), &unsolved_monkeys)
    );
}
