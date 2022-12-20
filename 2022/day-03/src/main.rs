use bitmaps::Bitmap;
use itertools::Itertools;
use std::{fs::File, io::Read};

fn byte_to_priority(byte: &u8) -> usize {
    let priority = match byte {
        97..=122 => byte - 97 + 1,
        65..=90 => byte - 65 + 27,
        _ => panic!(),
    };
    priority as usize
}

fn calc_rucksack_priority(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut bitmap: Bitmap<64> = Bitmap::new();
    for byte in &bytes[0..bytes.len() / 2] {
        let priority = byte_to_priority(byte);
        bitmap.set(priority, true);
    }

    for byte in &bytes[bytes.len() / 2..] {
        let priority = byte_to_priority(byte);
        if bitmap.get(priority) {
            return priority;
        }
    }

    panic!()
}

fn rucksack_bitmap(input: &str) -> Bitmap<64> {
    let bytes = input.trim().as_bytes();

    let mut bitmap: Bitmap<64> = Bitmap::new();
    for byte in bytes {
        let priority = byte_to_priority(byte);
        bitmap.set(priority, true);
    }

    bitmap
}

fn find_badge_item(bitmaps: [Bitmap<64>; 3]) -> usize {
    let [map1, map2, map3] = bitmaps;
    let badge_map = map1 & map2 & map3;
    badge_map.first_index().unwrap()
}

fn rucksack_reorganization(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| calc_rucksack_priority(line.trim()))
        .sum()
}

fn rucksack_badges(input: &str) -> usize {
    let mut sum = 0;
    for chunk in &input.split("\n").into_iter().chunks(3) {
        let bitmaps: Vec<_> = chunk.map(|line| rucksack_bitmap(line)).collect();
        sum += find_badge_item(bitmaps.try_into().unwrap());
    }

    sum
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", rucksack_reorganization(&input));
    println!("Part Two: {}", rucksack_badges(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn byte_to_priority_works() {
        assert_eq!(byte_to_priority(&('p' as u8)), 16);
        assert_eq!(byte_to_priority(&('L' as u8)), 38);
    }

    #[test]
    fn example_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let part_one = rucksack_reorganization(input);
        assert_eq!(part_one, 157);

        let part_two = rucksack_badges(input);
        assert_eq!(part_two, 70);
    }
}
