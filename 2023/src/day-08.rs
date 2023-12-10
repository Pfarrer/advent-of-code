use std::{collections::HashMap, fs::read_to_string};

struct Map {
    directions: Vec<char>,
    network: HashMap<String, (String, String)>,
}

fn main() {
    let input = read_to_string("./inputs/day-08.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    // println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let map = parser::parse(input);
    todo!()
}

fn solve_part2(input: &str) -> usize {
    todo!()
}

mod parser {
    use super::*;

    pub(super) fn parse(input: &str) -> Map {
        let mut lines = input.lines();

        let directions = lines.next().unwrap().chars().collect();
        lines.next();

        let network = lines.map(|line| parse_network_line(line)).collect();

        Map {
            directions,
            network,
        }
    }

    fn parse_network_line(line: &str) -> (String, (String, String)) {
        let (node_name, directions) = line.split_once(" = ").unwrap();
        let (direction_l, direction_r) = directions
            .trim_matches('(')
            .trim_matches(')')
            .split_once(", ")
            .unwrap();

        (
            node_name.to_owned(),
            (direction_l.to_owned(), direction_r.to_owned()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_example1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part1_works_example2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)        
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 6);
    }

    // #[test]
    // fn part2_works() {
    //     let solution = solve_part2(&INPUT);
    //     assert_eq!(solution, );
    // }
}
