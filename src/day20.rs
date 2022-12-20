use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day20() {
    let f: File = File::open("data/day20.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut orig_pos_to_val: Vec<i128> = vec![];
    let mut orig_pos_to_new_pos: HashMap<usize, usize> = HashMap::new();
    let mut new_pos_to_orig_pos: Vec<usize> = vec![];
    let mixing_len = input_data.len();
    let cycle_len: i128 = mixing_len as i128 - 1;
    let mut orig_zero_index: usize = 0;
    for (index, line) in input_data.iter().enumerate() {
        let val = line.parse::<i128>().unwrap();
        if val == 0 {
            orig_zero_index = index;
        }
        orig_pos_to_val.push(val * 811589153);
        orig_pos_to_new_pos.insert(index, index);
        new_pos_to_orig_pos.insert(index, index);
    }

    for _ in 0..10 {
        for (mixing_index, val) in orig_pos_to_val.iter().enumerate().take(mixing_len) {
            let mut move_val = val % cycle_len;
            let starting_position: i128 = *orig_pos_to_new_pos.get(&mixing_index).unwrap() as i128;
            if starting_position + move_val <= 0 {
                move_val += cycle_len;
            } else if (starting_position + move_val) > cycle_len {
                move_val -= cycle_len;
            }
            let new_position = (move_val + starting_position) as usize;

            if move_val >= 0 {
                for ii in starting_position..(starting_position + move_val) {
                    new_pos_to_orig_pos[ii as usize] = new_pos_to_orig_pos[ii as usize + 1];
                    orig_pos_to_new_pos.insert(new_pos_to_orig_pos[ii as usize], ii as usize);
                }
            } else {
                for ii in ((starting_position + move_val)..(starting_position)).rev() {
                    new_pos_to_orig_pos[ii as usize + 1] = new_pos_to_orig_pos[ii as usize];
                    orig_pos_to_new_pos
                        .insert(new_pos_to_orig_pos[ii as usize + 1], ii as usize + 1);
                }
            }
            orig_pos_to_new_pos.insert(mixing_index, new_position);
            new_pos_to_orig_pos[new_position] = mixing_index;
        }
    }

    let new_zero_index = orig_pos_to_new_pos.get(&orig_zero_index).unwrap();
    println!(
        "Grove coordinates are {}",
        orig_pos_to_val[*new_pos_to_orig_pos
            .get((new_zero_index + 1000) % mixing_len)
            .unwrap()]
            + orig_pos_to_val[*new_pos_to_orig_pos
                .get((new_zero_index + 2000) % mixing_len)
                .unwrap()]
            + orig_pos_to_val[*new_pos_to_orig_pos
                .get((new_zero_index + 3000) % mixing_len)
                .unwrap()]
    );
}
