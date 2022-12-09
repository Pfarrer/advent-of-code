use std::{fs::File, io::Read};

mod parser;

pub type SupplyStack = Vec<char>;

#[derive(Debug)]
pub struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn execute_instructions(stacks: &mut Vec<SupplyStack>, instructions: &Vec<Instruction>, crane_capacity: usize) {
    for instruction in instructions {
        for i in (0..instruction.count).step_by(crane_capacity) {
            let mut crane_bay = Vec::new();
            for _ in 0..std::cmp::min(instruction.count, crane_capacity) {
                let value = stacks[instruction.from-1].pop().unwrap();
                crane_bay.push(value);
            }
            
            for value in crane_bay.iter().rev() {
                stacks[instruction.to-1].push(*value);
            }
        }
    }
}

fn get_top_stack_items(stacks: &Vec<SupplyStack>) -> String {
    stacks.iter().filter_map(|stack| stack.last()).into_iter().collect()
}

fn rearrange_stack(input: &str, crane_capacity: usize) -> String {
    let (mut stacks, instructions) = parser::parse(input);
    execute_instructions(&mut stacks, &instructions, crane_capacity);
    get_top_stack_items(&stacks)
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", rearrange_stack(&input, 1));
    println!("Part Two: {}", rearrange_stack(&input, usize::MAX));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let part_one = rearrange_stack(input, 1);
        assert_eq!(part_one, "CMZ");

        let part_two = rearrange_stack(input, usize::MAX);
        assert_eq!(part_two, "MCD");
    }
}