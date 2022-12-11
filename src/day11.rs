use std::collections::VecDeque;

const USE_REAL_INPUT: bool = true;

enum Operation {
    Plus,
    Times,
    Squared,
}

struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    operation_value: u128,
    test: u128,
    test_if_true: usize,
    test_if_false: usize,
}

impl Monkey {
    fn process_item(&mut self) -> (usize, u128) {
        let mut worry_level = self.items.pop_front().unwrap();
        worry_level = match self.operation {
            Operation::Plus => worry_level + self.operation_value,
            Operation::Times => worry_level * self.operation_value,
            Operation::Squared => worry_level * worry_level,
        };
        if worry_level % self.test == 0 {
            (self.test_if_true, worry_level)
        } else {
            (self.test_if_false, worry_level)
        }
    }
}

// Too lazy to parse input, hardcode all the monkeys here
fn populate_monkeys(monkeys: &mut Vec<Monkey>) {
    if !USE_REAL_INPUT {
        let example_monkey0 = Monkey {
            items: VecDeque::from([79, 98]),
            operation: Operation::Times,
            operation_value: 19,
            test: 23,
            test_if_true: 2,
            test_if_false: 3,
        };
        let example_monkey1 = Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: Operation::Plus,
            operation_value: 6,
            test: 19,
            test_if_true: 2,
            test_if_false: 0,
        };
        let example_monkey2 = Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: Operation::Squared,
            operation_value: 0,
            test: 13,
            test_if_true: 1,
            test_if_false: 3,
        };
        let example_monkey3 = Monkey {
            items: VecDeque::from([74]),
            operation: Operation::Plus,
            operation_value: 3,
            test: 17,
            test_if_true: 0,
            test_if_false: 1,
        };
        monkeys.push(example_monkey0);
        monkeys.push(example_monkey1);
        monkeys.push(example_monkey2);
        monkeys.push(example_monkey3);
    } else {
        let real_monkey0 = Monkey {
            items: VecDeque::from([57]),
            operation: Operation::Times,
            operation_value: 13,
            test: 11,
            test_if_true: 3,
            test_if_false: 2,
        };
        let real_monkey1 = Monkey {
            items: VecDeque::from([58, 93, 88, 81, 72, 73, 65]),
            operation: Operation::Plus,
            operation_value: 2,
            test: 7,
            test_if_true: 6,
            test_if_false: 7,
        };
        let real_monkey2 = Monkey {
            items: VecDeque::from([65, 95]),
            operation: Operation::Plus,
            operation_value: 6,
            test: 13,
            test_if_true: 3,
            test_if_false: 5,
        };
        let real_monkey3 = Monkey {
            items: VecDeque::from([58, 80, 81, 83]),
            operation: Operation::Squared,
            operation_value: 0,
            test: 5,
            test_if_true: 4,
            test_if_false: 5,
        };
        let real_monkey4 = Monkey {
            items: VecDeque::from([58, 89, 90, 96, 55]),
            operation: Operation::Plus,
            operation_value: 3,
            test: 3,
            test_if_true: 1,
            test_if_false: 7,
        };
        let real_monkey5 = Monkey {
            items: VecDeque::from([66, 73, 87, 58, 62, 67]),
            operation: Operation::Times,
            operation_value: 7,
            test: 17,
            test_if_true: 4,
            test_if_false: 1,
        };
        let real_monkey6 = Monkey {
            items: VecDeque::from([85, 55, 89]),
            operation: Operation::Plus,
            operation_value: 4,
            test: 2,
            test_if_true: 2,
            test_if_false: 0,
        };
        let real_monkey7 = Monkey {
            items: VecDeque::from([73, 80, 54, 94, 90, 52, 69, 58]),
            operation: Operation::Plus,
            operation_value: 7,
            test: 19,
            test_if_true: 6,
            test_if_false: 0,
        };
        monkeys.push(real_monkey0);
        monkeys.push(real_monkey1);
        monkeys.push(real_monkey2);
        monkeys.push(real_monkey3);
        monkeys.push(real_monkey4);
        monkeys.push(real_monkey5);
        monkeys.push(real_monkey6);
        monkeys.push(real_monkey7);
    }
}

pub(crate) fn day11() {
    let mut monkeys: Vec<Monkey> = vec![];
    populate_monkeys(&mut monkeys);

    // Keep worry levels under control by coming up with a magic number
    let mut magic_worry_number = 1;
    for monkey in &monkeys {
        magic_worry_number *= monkey.test;
    }

    let mut monkey_activity_counts: Vec<u128> = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkey_activity_counts[i] += 1;
                let (new_monkey, item) = monkeys[i].process_item();
                monkeys[new_monkey]
                    .items
                    .push_back(item % magic_worry_number);
            }
        }
    }
    monkey_activity_counts.sort();
    monkey_activity_counts.reverse();
    println!(
        "Monkey business score: {}",
        monkey_activity_counts.first().unwrap() * monkey_activity_counts.get(1).unwrap()
    );
}
