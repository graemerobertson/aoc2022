use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day04() {
    let f: File = File::open("data/day04.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let list_of_section_assignments: Vec<String> =
        reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut count_of_pairs_with_complete_overlap: usize = 0;
    let mut count_of_pairs_with_any_overlap: usize = 0;
    for section_assignment_pair in list_of_section_assignments {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
        }
        let cap = RE.captures(&section_assignment_pair).unwrap();
        let mut elf1_assignments: HashSet<u32> = HashSet::new();
        let mut elf2_assignments: HashSet<u32> = HashSet::new();
        for i in cap[1].parse::<u32>().unwrap()..cap[2].parse::<u32>().unwrap() + 1 {
            elf1_assignments.insert(i);
        }
        for i in cap[3].parse::<u32>().unwrap()..cap[4].parse::<u32>().unwrap() + 1 {
            elf2_assignments.insert(i);
        }
        if elf1_assignments.is_subset(&elf2_assignments)
            || elf2_assignments.is_subset(&elf1_assignments)
        {
            count_of_pairs_with_complete_overlap += 1;
        }
        if !elf1_assignments.is_disjoint(&elf2_assignments) {
            count_of_pairs_with_any_overlap += 1;
        }
    }
    println!(
        "Number of pairs with a complete overlap is {}",
        count_of_pairs_with_complete_overlap
    );
    println!(
        "Number of pairs with some overlap is {}",
        count_of_pairs_with_any_overlap
    );
}
