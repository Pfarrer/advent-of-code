use std::collections::HashMap;

use crate::{CommandLine, LsLine};

pub fn build(commands: &Vec<CommandLine>) -> HashMap<Vec<String>, usize> {
    let mut map = HashMap::new();

    let mut current_path = vec!["/".to_string()];
    for command in commands {
        match command {
            CommandLine::CdCommand(ref dir_name) if dir_name == "/" => current_path = vec!["/".to_string()],
            CommandLine::CdCommand(ref dir_name) if dir_name == ".." => {
                current_path.pop();
            },
            CommandLine::CdCommand(ref dir_name) => current_path.push(dir_name.to_string()),
            CommandLine::LsCommand(ref ls_lines) => {
                let total_file_size = ls_lines.iter().fold(0, |acc, line| {
                    if let LsLine::File(_, size) = line {
                        acc + size
                    } else { acc }
                });
                
                let mut update_path = current_path.clone();
                while !update_path.is_empty() {
                    *map.entry(update_path.clone()).or_insert(0) += total_file_size;
                    update_path.pop();
                }
            }
        };
    }
    
    map
}