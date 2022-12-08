use std::{fs::File, io::Read, ops::RangeInclusive, cmp};

use regex::Regex;

fn extract_ranges(input: &str)-> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let re = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();

    let capture = re.captures(input).unwrap();

    let range1 = capture.get(1).unwrap().as_str().parse::<usize>().unwrap()..=capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let range2 = capture.get(3).unwrap().as_str().parse::<usize>().unwrap()..=capture.get(4).unwrap().as_str().parse::<usize>().unwrap();

    (range1, range2)
}

fn is_range_fully_contained(input: &str) -> bool {
    let (range1, range2) = extract_ranges(input);

    if range1.start() == range2.start() || range1.end() == range2.end() {
        return true;
    }

    let (first_range, second_range) = if range1.start() < range2.start() {
        (range1, range2)
    } else {
        (range2, range1)
    };

    first_range.end() > second_range.end()
}

fn is_range_overlapping(input: &str) -> bool {
    let (range1, range2) = extract_ranges(input);
    
    // https://stackoverflow.com/a/12888920
    cmp::max(range1.start(), range2.start()) <= cmp::min(range1.end(), range2.end())
}

fn count_ranges_fully_contained(input: &str) -> usize {
    input.trim().split("\n").filter(|line| is_range_fully_contained(line.trim())).count()
}

fn count_overlapping_ranges(input: &str) -> usize {
    input.trim().split("\n").filter(|line| is_range_overlapping(line.trim())).count()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", count_ranges_fully_contained(&input));
    println!("Part Two: {}", count_overlapping_ranges(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        let part_one = count_ranges_fully_contained(input);
        assert_eq!(part_one, 2);

        let part_two = count_overlapping_ranges(input);
        assert_eq!(part_two, 4);
    }

    #[test]
    fn custom_example_works() {
        let input = "2-4,1-5
        2-4,1-3
        2-4,3-5
        2-5,3-4";

        let part_two = count_overlapping_ranges(input);
        assert_eq!(part_two, 4);
    }
}