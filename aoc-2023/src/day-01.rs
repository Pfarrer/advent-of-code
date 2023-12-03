use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./inputs/day-01.txt").expect("read input");
    let solution = solve(&input);
    println!("Solution: {}", solution);
}

fn solve(input: &str) -> usize {
    let re = Regex::new(r"^[^\d]*(?<first>\d+).*(?<last>\d+)?[^\d]*$").unwrap();

    input.lines().into_iter().map(|line| {
        let capture = re.captures(line).expect(format!("numbers not found in line: {}", line).as_str());
        let first: usize = capture.name("first").unwrap().as_str().parse().unwrap();
        let last: usize = capture.name("last").map_or(first, |v| v.as_str().parse().unwrap());
        
        let res = format!("{}{}", first, last).parse::<usize>().unwrap();
        dbg!(line, first, last, res);
        res
    }).sum()
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
        let solution = solve(&input);
        assert_eq!(solution, 142);
    }
}
