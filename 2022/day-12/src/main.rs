use std::{fs::File, io::Read};

use pathfinding::prelude::*;

type Position = (usize, usize);
type HightMap = Vec<Vec<u8>>;

fn input_char_to_height(c: char) -> u8 {
    match c {
        'S' => 1,
        'E' => 26,
        'a'..='z' => c as u8 - 96,
        _ => panic!(),
    }
}

fn parse_height_values(input: &str) -> HightMap {
    let lines: Vec<_> = input.trim().split("\n").collect();
    lines
        .iter()
        .copied()
        .map(|line| line.chars().map(|c| input_char_to_height(c)).collect())
        .collect()
}

fn parse_start_goal_positions(input: &str, columns: usize) -> (Position, Position) {
    let start_n = input.find('S').unwrap();
    let goal_n = input.find('E').unwrap();

    (
        (start_n / columns, start_n % columns - start_n / columns),
        (goal_n / columns, goal_n % columns - goal_n / columns),
    )
}

fn parse_input(input: &str) -> (HightMap, (Position, Position)) {
    let height_map: HightMap = parse_height_values(input);
    let start_goal_positions = parse_start_goal_positions(input, height_map[0].len());

    (height_map, start_goal_positions)
}

fn is_successor(map: &HightMap, from: Position, to: Position) -> bool {
    let from_height_opt = map.get(from.0).and_then(|row| row.get(from.1));
    let to_height_opt = map.get(to.0).and_then(|row| row.get(to.1));

    match (from_height_opt, to_height_opt) {
        (Some(from_height), Some(to_height)) => {
            from_height >= to_height || from_height + 1 == *to_height
        }
        _ => false,
    }
}

fn successors(map: &HightMap, pos: Position) -> Vec<(Position, usize)> {
    let mut neighbors = vec![];

    if pos.0 > 0 && is_successor(map, pos, (pos.0 - 1, pos.1)) {
        neighbors.push(((pos.0 - 1, pos.1), 1));
    }
    if is_successor(map, pos, (pos.0 + 1, pos.1)) {
        neighbors.push(((pos.0 + 1, pos.1), 1));
    }

    if pos.1 > 0 && is_successor(map, pos, (pos.0, pos.1 - 1)) {
        neighbors.push(((pos.0, pos.1 - 1), 1));
    }
    if is_successor(map, pos, (pos.0, pos.1 + 1)) {
        neighbors.push(((pos.0, pos.1 + 1), 1));
    }

    neighbors
}

fn shortest_path_length(map: &HightMap, start: Position, goal: Position) -> Option<usize> {
    astar(
        &start,
        |p| successors(map, *p),
        |p| ((p.0 as i32 - goal.0 as i32).abs() + (p.1 as i32 - goal.1 as i32).abs()) as usize,
        |p| *p == goal,
    )
    .map(|(_, steps)| steps)
}

/// Simply route from all possible start locations and take the minimum path length.
/// Not the smartest solution, but does the trick.
fn shortest_path_from_any_a_length(map: &HightMap, goal: Position) -> usize {
    let start_positions: Vec<Position> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .copied()
                .enumerate()
                .filter(|(_, height)| *height == 1)
                .map(|(x, _)| (y, x))
                .collect::<Vec<Position>>()
        })
        .collect();

    start_positions
        .iter()
        .filter_map(|start| shortest_path_length(map, *start, goal))
        .min()
        .unwrap_or(usize::MAX)
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let (map, (start, goal)) = parse_input(&input);
    println!("Part One: {:?}", shortest_path_length(&map, start, goal));
    println!("Part Two:\n{}", shortest_path_from_any_a_length(&map, goal));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        let (map, (start, goal)) = parse_input(EXAMPLE_INPUT);
        let result = shortest_path_length(&map, start, goal);
        assert_eq!(result, Some(31));
    }

    #[test]
    fn part2_works() {
        let (map, (_, goal)) = parse_input(EXAMPLE_INPUT);
        let result = shortest_path_from_any_a_length(&map, goal);
        assert_eq!(result, 29);
    }

    #[test]
    fn input_char_to_height_works() {
        assert_eq!(input_char_to_height('a'), 1);
        assert_eq!(input_char_to_height('c'), 3);
        assert_eq!(input_char_to_height('z'), 26);

        assert_eq!(input_char_to_height('S'), 1);
        assert_eq!(input_char_to_height('E'), 26);
    }

    #[test]
    fn parse_start_goal_positions_works() {
        let (start, goal) = parse_start_goal_positions(EXAMPLE_INPUT, 8);
        assert_eq!(start, (0, 0));
        assert_eq!(goal, (2, 5));
    }
}
