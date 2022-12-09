use nom::{*, bytes::complete::tag, character::complete::{anychar, digit1, newline}, multi::separated_list0, combinator::opt};

use crate::{SupplyStack, Instruction};

pub fn parse(input: &str) -> (Vec<SupplyStack>, Vec<Instruction>) {
    let input_sections: Vec<_> = input.split("\n\n").collect();

    let (_, stacks) = supply_stacks(input_sections[0]).unwrap();
    let (_, instructions) = instructions(input_sections[1]).unwrap();
    
    (stacks, instructions)
}

fn supply_stacks(input: &str) -> IResult<&str, Vec<SupplyStack>> {
    let lines:Vec<_> = input.split("\n").collect();
    let mut lines_iter = lines.iter().rev();
    
    let stack_numbers_line = lines_iter.next().unwrap();
    let stack_count = stack_numbers_line.split_whitespace().count();

    let mut stacks: Vec<SupplyStack> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for line in lines_iter {
        let (_, crates) = nom::multi::many1(stack_line)(line)?;
        for (i, stack_crate) in crates.iter().enumerate() {
            if let Some(value) = stack_crate {
                stacks[i].push(*value);
            }
        }
    }

    Ok((input, stacks))
}

fn stack_line(input: &str) -> IResult<&str, Option<char>> {
    nom::sequence::terminated(stack_crate, opt(tag(" ")))(input)
}

fn stack_crate(input: &str) -> IResult<&str, Option<char>> {
    nom::branch::alt((filled_crate, empty_crate))(input)
}

fn empty_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn filled_crate(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("[")(input)?;
    let (input, cargo) = anychar(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Some(cargo)))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(newline, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, count_digits) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_digits) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_digits) = digit1(input)?;

    let from = from_digits.parse().unwrap();
    let to = to_digits.parse().unwrap();
    let count = count_digits.parse().unwrap();

    Ok((input, Instruction { from, to, count}))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 3
move 1 from 1 to 2";

        let (stacks, instructions) = parse(input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(instructions.len(), 4);
    }

    #[test]
    fn supply_stacks_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        let (_, stacks) = supply_stacks(input).unwrap();
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[2].len(), 1);
    }
}