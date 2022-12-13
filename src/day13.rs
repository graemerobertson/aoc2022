use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn convert_string_to_list(input_str: &str) -> Vec<String> {
    let mut output_list: Vec<String> = vec![];
    let mut depth = 0;
    let mut elem: String = "".to_string();

    for c in input_str.chars() {
        match c {
            '[' => {
                if depth > 0 {
                    elem.push(c)
                };
                depth += 1
            }
            ']' => {
                if depth == 1 {
                    output_list.push(elem.clone());
                }
                {
                    elem.push(c);
                };
                depth -= 1
            }
            ',' => {
                if depth == 1 {
                    output_list.push(elem.clone());
                    elem = "".to_string();
                } else {
                    elem.push(c);
                }
            }
            _ => elem.push(c),
        }
    }
    output_list
}

fn cmp_packets(left_packet: &str, right_packet: &str) -> Ordering {
    if left_packet.parse::<u32>().is_ok() && right_packet.parse::<u32>().is_ok() {
        // Two integers
        left_packet
            .parse::<u32>()
            .unwrap()
            .cmp(&right_packet.parse::<u32>().unwrap())
    } else if left_packet.parse::<u32>().is_ok() {
        // An integer and a list - convert the integer to a trivial list and recurse
        cmp_packets(&format!("[{}]", left_packet), right_packet)
    } else if right_packet.parse::<u32>().is_ok() {
        // An integer and a list - convert the integer to a trivial list and recurse
        cmp_packets(left_packet, &format!("[{}]", right_packet))
    } else {
        // Two lists - recurse over the elements of the lists
        let left_packet_list: Vec<String> = convert_string_to_list(left_packet);
        let right_packet_list: Vec<String> = convert_string_to_list(right_packet);
        let mut ordering: Ordering = Ordering::Equal;
        for (index, left_packet_list_item) in left_packet_list.iter().enumerate() {
            if right_packet_list.get(index).is_some() {
                ordering =
                    cmp_packets(left_packet_list_item, right_packet_list.get(index).unwrap());
                if ordering != Ordering::Equal {
                    return ordering;
                }
            } else {
                return Ordering::Greater;
            }
        }
        if ordering == Ordering::Equal && right_packet_list.len() > left_packet_list.len() {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

pub(crate) fn day13() {
    let f: File = File::open("data/day13.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut indices_sum: usize = 0;
    for (index, chunk) in input_data.chunks(3).enumerate() {
        if cmp_packets(chunk.get(0).unwrap(), chunk.get(1).unwrap()) == Ordering::Less {
            indices_sum += index + 1;
        }
    }
    println!(
        "Sum of indices of correctly ordered pairs is {}",
        indices_sum
    );

    let mut ordered_packets: Vec<String> =
        input_data.into_iter().filter(|x| !x.is_empty()).collect();
    ordered_packets.push("[[2]]".to_string());
    ordered_packets.push("[[6]]".to_string());
    ordered_packets.sort_by(|a, b| cmp_packets(a, b));
    let mut decoder_key: u32 = 1;
    for (index, packet) in ordered_packets.iter().enumerate() {
        if packet == "[[2]]" || packet == "[[6]]" {
            decoder_key *= 1 + index as u32;
        }
    }
    println!("Decoder key for the distress signal is {}", decoder_key);
}
