use std::{fs::File, io::Read};

mod parser;
mod map_builder;

#[derive(Debug, PartialEq)]
pub enum CommandLine {
    CdCommand(String),
    LsCommand(Vec<LsLine>),
}

#[derive(Debug, PartialEq)]
pub enum LsLine {
    File(String, usize),
    Dir(String),
}

/// Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?
fn sum_smaller_folders(input: &str) -> usize {
    let commands = parser::parse(input);
    let map = map_builder::build(&commands);
    map.values().cloned().filter(|size| *size <= 100000).sum()
}

/// Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
fn smallest_possible_folder_to_delete(input: &str) -> usize {
    const DISK_SIZE: usize = 70000000;
    const UPDATE_SIZE: usize = 30000000;

    let commands = parser::parse(input);
    let map = map_builder::build(&commands);

    let disk_used = map.get(&vec!["/".to_owned()]).unwrap();
    let disk_size_needed = UPDATE_SIZE - (DISK_SIZE - disk_used);

    let mut dir_sizes: Vec<_> = map.values().collect();
    dir_sizes.sort();
    
    for dir_size in dir_sizes {
        if *dir_size >= disk_size_needed {
            return *dir_size;
        }
    }
    
    panic!()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", sum_smaller_folders(&input));
    println!("Part Two: {}", smallest_possible_folder_to_delete(&input));
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    pub const EXAMPLE_INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn example_works() {
        let part_one = sum_smaller_folders(EXAMPLE_INPUT);
        assert_eq!(part_one, 95437);

        let part_two = smallest_possible_folder_to_delete(EXAMPLE_INPUT);
        assert_eq!(part_two, 24933642);
    }
}