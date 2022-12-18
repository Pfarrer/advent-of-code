use std::{fs::File, io::Read};

#[derive(Copy, Clone)]
enum HeadMovement {
    UP, DOWN, LEFT, RIGHT
}

type Position = (i64, i64);

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

fn move_head(head_pos: &mut Position, movement: &HeadMovement) {
    match movement {
        HeadMovement::UP => head_pos.1 = head_pos.1 + 1,
        HeadMovement::DOWN => head_pos.1 = head_pos.1 - 1,
        HeadMovement::LEFT => head_pos.0 = head_pos.0 - 1,
        HeadMovement::RIGHT => head_pos.0 = head_pos.0 + 1,
    }
}

fn is_neighboring_point(p1: &Position, p2: &Position) -> bool {
    let x_diff = p1.0 - p2.0;
    let y_diff = p1.1 - p2.1;

    f64::sqrt((x_diff * x_diff + y_diff * y_diff) as f64) < 2.
}

fn move_tail(head_pos: &Position, tail_pos: &Position) -> Option<Position> {
    if is_neighboring_point(head_pos, tail_pos) {
        return None;
    }
    
    let mut moved_tail_pos = tail_pos.clone();

    if head_pos.0 > tail_pos.0 {
        moved_tail_pos.0 += 1;
    } else if head_pos.0 < tail_pos.0 {
        moved_tail_pos.0 -= 1;
    }

    if head_pos.1 > tail_pos.1 {
        moved_tail_pos.1 += 1;
    } else if head_pos.1 < tail_pos.1 {
        moved_tail_pos.1 -= 1;
    }

    Some(moved_tail_pos)
}

fn simulate_rope_positions(head_movements: &Vec<HeadMovement>, mut rope: Vec<Position>) -> Vec<Position> {
    let mut tail_positions = vec!(rope.last().unwrap().clone());

    for movement in head_movements {
        move_head(rope.first_mut().unwrap(), movement);

        for head_index in 0..rope.len()-1 {
            if let Some(moved_tail_pos) = move_tail(&rope[head_index], &rope[head_index+1]) {
                rope[head_index+1] = moved_tail_pos;
            }
        }

        tail_positions.push(rope.last().unwrap().clone());
    }

    tail_positions
}

fn count_unique_tail_positions(head_movements: &Vec<HeadMovement>, rope: Vec<Position>) -> usize {
    let  mut tail_positions = simulate_rope_positions(&head_movements, rope);

    tail_positions.sort();
    tail_positions.dedup();
    tail_positions.len()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let head_movements = parse_input(&input);
    println!("Part One: {}", count_unique_tail_positions(&head_movements, [(0, 0)].repeat(2)));
    println!("Part Two: {}", count_unique_tail_positions(&head_movements, [(0, 0)].repeat(10)));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_LARGER_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_works() {
        let head_movements = parse_input(EXAMPLE_INPUT);
        let result = count_unique_tail_positions(&head_movements, [(0, 0)].repeat(2));
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_works_simple() {
        let head_movements = parse_input(EXAMPLE_INPUT);
        let result_simple = count_unique_tail_positions(&head_movements, [(0, 0)].repeat(10));
        assert_eq!(result_simple, 1);
    }

    #[test]
    fn part2_works_larger() {
        let head_movements = parse_input(EXAMPLE_LARGER_INPUT);
        let result_larger = count_unique_tail_positions(&head_movements, [(0, 0)].repeat(10));
        assert_eq!(result_larger, 36);
    }

    #[test]
    fn is_neighboring_point_works() {
        assert!(is_neighboring_point(&(2, 2), &(1, 1)));
        assert!(is_neighboring_point(&(2, 2), &(1, 2)));
        assert!(is_neighboring_point(&(2, 2), &(2, 1)));
        assert!(is_neighboring_point(&(2, 2), &(2, 2)));
        assert!(is_neighboring_point(&(2, 2), &(2, 3)));
        assert!(is_neighboring_point(&(2, 2), &(3, 2)));
        assert!(is_neighboring_point(&(2, 2), &(3, 3)));

        assert!(!is_neighboring_point(&(1, 2), &(3, 3)));
    }

    #[test]
    fn move_tail_works() {
        assert_eq!(move_tail(&(2, 2), &(1, 1)), None);

        assert_eq!(move_tail(&(2, 2), &(2, 0)), Some((2, 1)));
        assert_eq!(move_tail(&(2, 2), &(0, 2)), Some((1, 2)));
        assert_eq!(move_tail(&(2, 2), &(0, 1)), Some((1, 2)));
    }
}