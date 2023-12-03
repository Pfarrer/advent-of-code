use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./inputs/day-01.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let re = Regex::new(r"\d").unwrap();

    input
        .lines()
        .into_iter()
        .map(|line| {
            let captures: Vec<usize> = re
                .captures_iter(line)
                .map(|c| c.get(0).unwrap().as_str())
                .map(|s| s.parse().unwrap())
                .collect();
            let first: &usize = captures.first().unwrap();
            let last: &usize = captures.last().unwrap();

            first * 10 + last
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();

    fn parse_capture(s: &str) -> usize {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => s.parse().unwrap(),
        }
    }

    fn line_captures(line: &str, re: &Regex) -> Vec<usize> {
        let mut res = Vec::new();
        for i in 0..line.len() {
            if let Some(capture) = re.captures_at(line, i) {
                if capture.get(0).unwrap().start() != i {
                    continue;
                }
                res.push(parse_capture(capture.get(0).unwrap().as_str()));
            }
        }

        res
    }

    input
        .lines()
        .into_iter()
        .map(|line| {
            let captures: Vec<usize> = line_captures(line, &re);
            let first: &usize = captures.first().unwrap();
            let last: &usize = captures.last().unwrap();

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 142);
    }

    #[test]
    fn part1_test1() {
        let solution = solve_part1("769twotwo6rv9");
        assert_eq!(solution, 79);
    }

    #[test]
    fn part2_works() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        let solution = solve_part2(&input);
        assert_eq!(solution, 281);
    }

    #[test]
    fn part2_test1() {
        let input = r#"eightwo1eightwo"#;
        let solution = solve_part2(&input);
        assert_eq!(solution, 82);
    }

    #[test]
    fn part2_test2() {
        let input = r#"hclv99two89nsfdfour4"#;
        let solution = solve_part2(&input);
        assert_eq!(solution, 94);
    }
}
