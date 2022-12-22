use std::{fs::File, io::Read};
use std::fmt::{Debug, Formatter, Write};

use tracing::instrument;

const CELL_EMPTY: char = '.';
const CELL_ROCK: char = '#';
const CELL_SAND: char = 'o';

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Scenario {
    map: Vec<Vec<char>>,
}

impl Scenario {
    #[instrument]
    pub fn set_cell(&mut self, pos: Point, value: char) {
        let (dim_x, dim_y) = self.dim();

        if dim_y <= pos.y {
            self.resize_y(pos.y + 1);
        }
        if dim_x <= pos.x {
            self.resize_x(pos.x + 1);
        }

        self.map[pos.y][pos.x] = value;
    }

    fn dim(&self) -> (usize, usize) {
        if self.map.len() > 0 {
            (self.map[0].len(), self.map.len())
        } else {
            (0, 0)
        }
    }

    fn resize_x(&mut self, target_dim_x: usize) {
        self.map.iter_mut()
            .for_each(|row| row.resize(target_dim_x, CELL_EMPTY));
    }
    fn resize_y(&mut self, target_dim_y: usize) {
        let (dim_x, _) = self.dim();
        self.map.resize(target_dim_y, vec![CELL_EMPTY; dim_x]);
    }
}

struct Simulation<'a> {
    scenario: &'a Scenario,
}

impl<'a> Simulation<'a> {
    pub fn new(scenario: &'a Scenario) -> Simulation {
        Simulation { scenario }
    }

    pub fn get_cell(&self, pos: Point) -> char {
        self.scenario.map[pos.y][pos.x]
    }
}

impl Debug for Simulation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (dim_x, dim_y) = self.scenario.dim();
        for y in 0..dim_y {
            for x in 0..dim_x {
                f.write_char(self.get_cell(Point { x, y }))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_input_position(input: &str) -> Point {
    let parts: Vec<_> = input.trim().split(",").collect();

    Point {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap()
    }
}

fn parse_input_line(input: &str) -> Vec<Point> {
    input.trim().split("->").map(|position_str| parse_input_position(position_str)).collect()
}

#[derive(Debug)]
struct PositionIter {
    increment: (i32, i32),
    current: Point,
    end: Point,
}

impl PositionIter {
    pub fn between_inclusive(start: Point, end: Point) -> PositionIter {
        let increment = PositionIter::find_increment(&start, &end);

        PositionIter {
            increment,
            current: Point {
                x: (start.x as i32 - increment.0) as usize,
                y: (start.y as i32 - increment.1) as usize,
            },
            end,
        }
    }

    fn find_increment(start: &Point, end: &Point) -> (i32, i32) {
        if start.x == end.x {
            let y = if start.y > end.y { -1 } else { 1 };
            (0, y)
        } else {
            let x = if start.x > end.x { -1 } else { 1 };
            (x, 0)
        }
    }
}

impl Iterator for PositionIter {
    type Item = Point;

    #[instrument(ret)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        self.current = Point {
            x: (self.current.x as i32 + self.increment.0) as usize,
            y: (self.current.y as i32 + self.increment.1) as usize,
        };

        Some(self.current)
    }
}

fn parse_input(input: &str) -> Scenario {
    let rock_paths: Vec<_> = input.trim().split("\n").map(|line| parse_input_line(line)).collect();

    let mut scenario = Scenario {
        map: vec![],
    };
    for path in rock_paths {
        path.windows(2).for_each(|window| {
            let pos_iter = PositionIter::between_inclusive(window[0].clone(), window[1].clone());
            for position in pos_iter {
                scenario.set_cell(position, CELL_ROCK);
            }
        })
    }
    scenario
}

fn simulate_falling_sand(scenario: &Scenario) -> Simulation {
    Simulation::new(scenario)
}

fn count_settled_sand_pieces(scenario: &Scenario) -> usize {
    let simulation = simulate_falling_sand(scenario);
    0
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let scenario = parse_input(&input);
    println!("Part One: {:?}", count_settled_sand_pieces(&scenario));
    // println!("Part Two:\n{}", shortest_path_from_any_a_length(&map, goal));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parsing_works() {
        let scenario = parse_input(EXAMPLE_INPUT);
        assert_eq!(scenario.dim(), (504, 10));

        assert_eq!(scenario.map[9][494], CELL_ROCK);
        assert_eq!(scenario.map[8][494], CELL_EMPTY);
    }

    #[test]
    fn part1_works() {
        let scenario = parse_input(EXAMPLE_INPUT);
        let result = count_settled_sand_pieces(&scenario);
        assert_eq!(result, 24);
    }

    #[test]
    fn part1_printable_works() {
        tracing_subscriber::fmt::init();

        let scenario = parse_input("4,0 -> 4,2 -> 2,2
9,0 -> 8,0 -> 8,5 -> 0,5");
        let result = count_settled_sand_pieces(&scenario);
        assert_eq!(result, 24);
    }

    // #[test]
    // fn part2_works() {
    //     let (map, (_, goal)) = parse_input(EXAMPLE_INPUT);
    //     let result = shortest_path_from_any_a_length(&map, goal);
    //     assert_eq!(result, 29);
    // }
}
