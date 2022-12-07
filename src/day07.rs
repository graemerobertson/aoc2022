use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Directory {
    subdirs: HashSet<String>,
    size_of_files: usize,
}

impl Directory {
    fn size(&self, all_dirs: &HashMap<String, Directory>) -> usize {
        let mut size: usize = self.size_of_files;
        for subdir in &self.subdirs {
            let subdir = all_dirs.get(subdir).unwrap();
            size += subdir.size(all_dirs);
        }
        size
    }
}

pub(crate) fn day07() {
    let f: File = File::open("data/day07.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_list: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut all_dirs: HashMap<String, Directory> = HashMap::new();
    let mut current_path: Vec<String> = vec![];

    for line in input_list {
        if line.starts_with("$ cd") {
            let cd_target: String = line.split_at(5).1.to_string();
            if cd_target == ".." {
                current_path.pop();
            } else {
                current_path.push(cd_target);
                let new_dir: Directory = Directory {
                    subdirs: HashSet::new(),
                    size_of_files: 0,
                };
                all_dirs.insert(current_path.join("/"), new_dir);
            }
        } else if line.starts_with("dir ") {
            // Add new subdir to current directory
            let sub_dir_name: String = line.split_at(4).1.to_string();
            let mut sub_dir_path = current_path.clone();
            sub_dir_path.push(sub_dir_name);
            all_dirs
                .get_mut(&current_path.join("/"))
                .unwrap()
                .subdirs
                .insert(sub_dir_path.join("/"));
        } else if !line.starts_with("$ ls") {
            // Add new file to current directory
            let file_size = line.split(' ').next().unwrap().parse::<usize>().unwrap();

            all_dirs
                .get_mut(&current_path.join("/"))
                .unwrap()
                .size_of_files += file_size;
        }
    }

    let mut size_of_small_dirs: usize = 0;
    for dir in all_dirs.values() {
        let dir_size = dir.size(&all_dirs);
        if dir_size <= 100000 {
            size_of_small_dirs += dir_size;
        }
    }
    println!(
        "Total size of dirs with size at most 100000 is {}",
        size_of_small_dirs
    );

    let size_of_file_system: usize = all_dirs.get("/").unwrap().size(&all_dirs);
    let size_we_need_to_free = size_of_file_system - 40000000;
    // An upper bound for the size of the dir we need to delete.
    let mut size_of_dir_to_delete = 70000000;
    for dir in all_dirs.values() {
        let dir_size = dir.size(&all_dirs);
        if dir_size > size_we_need_to_free && dir_size < size_of_dir_to_delete {
            size_of_dir_to_delete = dir_size;
        }
    }
    println!("Delete dir with size {}", size_of_dir_to_delete);
}
