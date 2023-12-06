use std::fs::read_to_string;

struct Number {
    value: usize,
    line: usize,
    start: usize,
    end: usize,
}

struct Symbol {
    char: char,
    line: usize,
    column: usize,
}

struct Blueprint {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn main() {
    let input = read_to_string("./inputs/day-03.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    // println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let blueprint = parser::parse(input);
    adjacent_numbers(&blueprint).iter().map(|n| n.value).sum()
}

fn solve_part2(input: &str) -> usize {
    todo!()
}

fn adjacent_numbers(blueprint: &Blueprint) -> Vec<Number> {}

mod parser {
    use regex::Regex;

    use super::*;

    pub(super) fn parse(input: &str) -> Blueprint {
        Blueprint {
            numbers: parse_numbers(input),
            symbols: parse_symbols(input),
        }
    }

        let number_re = Regex::new(r"\d+").unwrap();
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 4361);
    }

    //     #[test]
    //     fn part2_works() {
    //         let input = r#"
    // "#;
    //         let solution = solve_part2(&input);
    //         assert_eq!(solution, );
    //     }
}
