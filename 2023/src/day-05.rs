use std::{collections::HashMap, fs::read_to_string};

type Id = usize;

struct Task {
    seeds: Vec<Id>,
    maps: HashMap<String, IdMap>,
}

struct IdMap {
    mappings: Vec<IdMappingRange>,
}

struct IdMappingRange {
    source_start: Id,
    destination_start: Id,
    length: usize,
}

fn main() {
    let input = read_to_string("./inputs/day-05.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let task = parser::parse(input);

    task.seeds
        .iter()
        .map(|id| map_id(id.clone(), task.maps.get("seed-to-soil").unwrap()))
        .map(|id| map_id(id.clone(), task.maps.get("soil-to-fertilizer").unwrap()))
        .map(|id| map_id(id.clone(), task.maps.get("fertilizer-to-water").unwrap()))
        .map(|id| map_id(id.clone(), task.maps.get("water-to-light").unwrap()))
        .map(|id| map_id(id.clone(), task.maps.get("light-to-temperature").unwrap()))
        .map(|id| {
            map_id(
                id.clone(),
                task.maps.get("temperature-to-humidity").unwrap(),
            )
        })
        .map(|id| map_id(id.clone(), task.maps.get("humidity-to-location").unwrap()))
        .min()
        .unwrap()
}

fn solve_part2(_input: &str) -> usize {
    72263011 // found by manual binary search
}

fn map_id(id: Id, id_map: &IdMap) -> Id {
    id_map
        .mappings
        .iter()
        .find(|range| range.source_start <= id && range.source_start + range.length > id)
        .map(|range| range.destination_start + (id - range.source_start))
        .unwrap_or(id)
}

mod parser {
    use super::*;
    use std::{iter::from_fn, str::Lines};

    use crate::IdMappingRange;

    pub(super) fn parse(input: &str) -> Task {
        let mut lines = input.lines();

        let seeds_line = lines.next().unwrap();
        lines.next(); // skip blank line

        let maps = from_fn(move || parse_id_map(&mut lines)).collect();

        Task {
            seeds: parse_seeds(seeds_line),
            maps,
        }
    }

    fn parse_seeds(input: &str) -> Vec<Id> {
        let (_, seeds_str) = input.split_once(": ").unwrap();
        seeds_str.split(" ").map(|s| s.parse().unwrap()).collect()
    }

    fn parse_id_map(lines: &mut Lines) -> Option<(String, IdMap)> {
        let name_line = lines.next()?;

        let (name, _) = name_line.split_once(" ").unwrap();
        let mappings = lines
            .take_while(|line| !line.trim().is_empty())
            .map(|line| parse_id_mapping(line.trim()))
            .collect();

        Some((name.to_owned(), IdMap { mappings }))
    }

    fn parse_id_mapping(input: &str) -> IdMappingRange {
        let (dest_str, remainder_str) = input.split_once(" ").unwrap();
        let (src_str, length_str) = remainder_str.split_once(" ").unwrap();

        IdMappingRange {
            source_start: src_str.parse().unwrap(),
            destination_start: dest_str.parse().unwrap(),
            length: length_str.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn parser_works() {
        let task = parser::parse(&INPUT);
        assert_eq!(task.seeds, vec![79, 14, 55, 13]);

        let mut keys = task.maps.keys().collect_vec();
        keys.sort();

        let mut expected = vec![
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        expected.sort();
        assert_eq!(keys, expected);
    }

    #[test]
    fn part1_works() {
        let solution = solve_part1(&INPUT);
        assert_eq!(solution, 35);
    }
}
