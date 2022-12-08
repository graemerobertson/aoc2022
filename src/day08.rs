use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day08() {
    let f: File = File::open("data/day08.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut grid: Vec<Vec<u32>> = vec![vec![0; input_data[0].len()]; input_data.len()];
    for (i, line) in input_data.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            grid[i][j] = point as u32 - 48;
        }
    }

    let mut count_of_visible_trees: u32 = 0;
    let mut best_scenic_score: u64 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let height_of_tree = grid[i][j];
            let mut current_scenic_score: u64 = 1;
            let mut count_of_blocked_views: u8 = 0;

            let mut inner_count_of_visible_trees = 0;
            for trees in grid.iter().rev().skip(grid.len() - i) {
                inner_count_of_visible_trees += 1;
                if height_of_tree <= trees[j] {
                    count_of_blocked_views += 1;
                    break;
                }
            }
            current_scenic_score *= inner_count_of_visible_trees;

            inner_count_of_visible_trees = 0;
            for trees in grid.iter().skip(i + 1) {
                inner_count_of_visible_trees += 1;
                if height_of_tree <= trees[j] {
                    count_of_blocked_views += 1;
                    break;
                }
            }
            current_scenic_score *= inner_count_of_visible_trees;

            inner_count_of_visible_trees = 0;
            for jj in (0..j).rev() {
                inner_count_of_visible_trees += 1;
                if height_of_tree <= grid[i][jj] {
                    count_of_blocked_views += 1;
                    break;
                }
            }
            current_scenic_score *= inner_count_of_visible_trees;

            inner_count_of_visible_trees = 0;
            for jj in j + 1..grid[0].len() {
                inner_count_of_visible_trees += 1;
                if height_of_tree <= grid[i][jj] {
                    count_of_blocked_views += 1;
                    break;
                }
            }
            current_scenic_score *= inner_count_of_visible_trees;

            if count_of_blocked_views != 4 {
                count_of_visible_trees += 1;
            }
            if current_scenic_score > best_scenic_score {
                best_scenic_score = current_scenic_score;
            }
        }
    }

    println!("{} trees are visible", count_of_visible_trees);
    println!(
        "{} is the highest scenic score for any tree",
        best_scenic_score
    );
}
