use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    line: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    char: char,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct Blueprint {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn main() {
    let input = read_to_string("./inputs/day-03.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let blueprint = parser::parse(input);
    adjacent_numbers(&blueprint).iter().map(|n| n.value).sum()
}

fn solve_part2(input: &str) -> usize {
    let blueprint = parser::parse(input);
    gear_ratios(&blueprint).iter().sum()
}

fn number_has_adjacent_symbol(num: &Number, symbols: &Vec<Symbol>) -> bool {
    symbols.iter().any(|sym| {
        num.line.abs_diff(sym.line) <= 1 && num.start <= sym.column + 1 && num.end + 1 >= sym.column
    })
}

fn adjacent_numbers(blueprint: &Blueprint) -> Vec<Number> {
    blueprint
        .numbers
        .iter()
        .cloned()
        .filter(|num| number_has_adjacent_symbol(num, &blueprint.symbols))
        .collect()
}

fn gear_ratios(blueprint: &Blueprint) -> Vec<usize> {
    let gears: Vec<_> = blueprint
        .symbols
        .iter()
        .cloned()
        .filter(|sym| sym.char == '*')
        .collect();

    gears
        .iter()
        .map(|sym| {
            let adjacent_numbers = blueprint
                .numbers
                .iter()
                .filter(|num| number_has_adjacent_symbol(num, &vec![sym.clone()]))
                .map(|num| num.value)
                .collect_vec();

            if adjacent_numbers.len() == 2 {
                adjacent_numbers[0] * adjacent_numbers[1]
            } else {
                0
            }
        })
        .collect_vec()
}

mod parser {
    use regex::Regex;

    use super::*;

    pub(super) fn parse(input: &str) -> Blueprint {
        Blueprint {
            numbers: parse_numbers(input),
            symbols: parse_symbols(input),
        }
    }

    fn parse_numbers(input: &str) -> Vec<Number> {
        let number_re = Regex::new(r"\d+").unwrap();
        input
            .lines()
            .enumerate()
            .flat_map(|(idx, line)| {
                number_re.captures_iter(line).map(move |cap| {
                    let num_match = cap.get(0).unwrap();
                    Number {
                        value: num_match.as_str().parse().unwrap(),
                        line: idx,
                        start: num_match.start(),
                        end: num_match.end() - 1,
                    }
                })
            })
            .collect()
    }

    fn parse_symbols(input: &str) -> Vec<Symbol> {
        let symbol_re = Regex::new(r"[^.\d]").unwrap();
        input
            .lines()
            .enumerate()
            .flat_map(|(idx, line)| {
                symbol_re.captures_iter(line).map(move |cap| {
                    let sym_match = cap.get(0).unwrap();
                    Symbol {
                        char: sym_match.as_str().chars().next().unwrap(),
                        line: idx,
                        column: sym_match.start(),
                    }
                })
            })
            .collect()
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

    #[test]
    fn part2_works() {
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
        let solution = solve_part2(&input);
        assert_eq!(solution, 467835);
    }
}
