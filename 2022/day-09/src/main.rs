use std::{fs::File, io::Read};

#[derive(Copy, Clone)]
enum HeadMovement {
    UP, DOWN, LEFT, RIGHT
}

fn parse_input_line(input: &str) -> Vec<HeadMovement> {
    let parts: Vec<_> = input.trim().split_whitespace().collect();
    
    let movement = match parts[0] {
        "U" => HeadMovement::UP,
        "D" => HeadMovement::DOWN,
        "L" => HeadMovement::LEFT,
        "R" => HeadMovement::RIGHT,
        _ => panic!(),
    };
    let count: usize = parts[1].parse().unwrap();
    
    [movement].repeat(count)
}

fn parse_input(input: &str) -> Vec<HeadMovement> {
    input.trim().split("\n").flat_map(|line| parse_input_line(line)).collect()
}

fn simulate_rope_positions(head_movements: &Vec<HeadMovement>) {

}

fn count_unique_tail_positions(head_movements: &Vec<HeadMovement>) -> usize {
    head_movements.len()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let head_movements = parse_input(&input);
    println!("Part One: {}", count_unique_tail_positions(&head_movements));
//    println!("Part Two: {}", rearrange_stack(&head_movements));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let head_movements = parse_input(&input);
        let part_one = count_unique_tail_positions(&head_movements);
        assert_eq!(part_one, 13);
    }
}