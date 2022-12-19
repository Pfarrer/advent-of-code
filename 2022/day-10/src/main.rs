use std::{fs::File, io::Read, ops::Range};

enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instruction(input: &str) -> Instruction {
    let parts: Vec<_> = input.trim().split_whitespace().collect();
    
    match parts[0] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::AddX(parts[1].parse().unwrap()),
        _ => panic!()
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split("\n").map(|line| parse_instruction(line)).collect()
}

fn simulate_register_value_by_cycle(program: &Vec<Instruction>) -> Vec<i32> {
    let mut value_by_cycle = vec![1];

    for instruction in program {
        let last_value = value_by_cycle.last().cloned().unwrap();
        value_by_cycle.push(last_value);
        match instruction {
            Instruction::AddX(add_value) => value_by_cycle.push(last_value + *add_value),
            Instruction::Noop => (),
        };
    }
    value_by_cycle
}

fn sum_signal_strengths(program: &Vec<Instruction>) -> i32 {
    let value_by_cycle = simulate_register_value_by_cycle(program);
    [20, 60, 100, 140, 180, 220].iter().map(|cycle| cycle * value_by_cycle[(*cycle-1) as usize]).sum()
}

fn sprite_range(x_value: i32) -> Range<i32> {
    x_value-1..x_value+2
}

fn render_crt_image(program: &Vec<Instruction>) -> String {
    const SCREEN_WIDTH: usize = 40;
    const SCREEN_HEIGHT: usize = 6;

    let value_by_cycle = simulate_register_value_by_cycle(program);
    let complete_string: Vec<char> = value_by_cycle.iter()
        .take(SCREEN_WIDTH * SCREEN_HEIGHT)
        .enumerate()
        .map(|(cycle, x_value)| if sprite_range(*x_value).contains(&((cycle % SCREEN_WIDTH) as i32)) { '#' } else { '.' })
        .collect();

    let lines: Vec<&[char]> = complete_string.chunks(SCREEN_WIDTH).collect();
    lines.iter().copied().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let program = parse_input(&input);
    println!("Part One: {}", sum_signal_strengths(&program));
    println!("Part Two:\n{}", render_crt_image(&program));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let program = parse_input(EXAMPLE_INPUT);
        let result = sum_signal_strengths(&program);
        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_works() {
        let program = parse_input(EXAMPLE_INPUT);
        let result = render_crt_image(&program);
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}