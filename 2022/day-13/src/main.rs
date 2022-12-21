use std::{fs::File, io::Read, vec};
use std::cmp::Ordering;

mod parser;

pub type PackagePair = (Package, Package);
pub type Package = Vec<Entry>;

#[derive(Debug, Clone)]
pub enum Entry {
    Number(u32),
    List(Vec<Entry>),
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entry::Number(self_number), Entry::Number(other_number)) => self_number == other_number,
            (Entry::List(_), Entry::Number(other_number)) => *self == Entry::List(vec![Entry::Number(*other_number)]),
            (Entry::Number(self_number), Entry::List(_)) => *other == Entry::List(vec![Entry::Number(*self_number)]),
            (Entry::List(self_list), Entry::List(other_list)) => {
                if self_list.len() != other_list.len() {
                    return false;
                }

                for (self_entry, other_entry) in self_list.iter().zip(other_list.iter()) {
                    if self_entry != other_entry {
                        return false;
                    }
                }

                return true;
            },
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Entry::Number(self_number), Entry::Number(other_number)) => self_number.partial_cmp(other_number),
            (Entry::List(_), Entry::Number(other_number)) => self.partial_cmp(&Entry::List(vec![Entry::Number(*other_number)])),
            (Entry::Number(self_number), Entry::List(_)) => Entry::List(vec![Entry::Number(*self_number)]).partial_cmp(other),
            (Entry::List(self_list), Entry::List(other_list)) => {
                for (self_entry, other_entry) in self_list.iter().zip(other_list.iter()) {
                    let result = self_entry.partial_cmp(other_entry);
                    if result != Some(Ordering::Equal) {
                        return result;
                    }
                }

                if self_list.len() == other_list.len() {
                    Some(Ordering::Equal)
                } else if self_list.len() < other_list.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            },
        }
    }
}

fn is_correctly_ordered(package_pair: &PackagePair) -> bool {
    package_pair.0 < package_pair.1
}

fn correctly_ordered_packages_index_sum(package_pairs: &Vec<PackagePair>) -> usize {
    package_pairs.iter()
        .enumerate()
        .filter(|(_, pair)| is_correctly_ordered(pair))
        .map(|(i, _)| i+1)
        .sum()
}

fn calc_decoder_key(package_pairs: &Vec<PackagePair>) -> usize {
    let divider_packet1 = vec![Entry::List(vec![Entry::Number(2)])];
    let divider_packet2 = vec![Entry::List(vec![Entry::Number(6)])];

    let mut all_packages:Vec<_> = package_pairs.iter().flat_map(|pair| vec![pair.0.clone(), pair.1.clone()]).collect();
    all_packages.append(&mut vec![divider_packet1.clone(), divider_packet2.clone()]);
    all_packages.sort_by(|left, right| left.partial_cmp(right).unwrap());

    let pos_divider_packet1 = all_packages.iter().position(|packet| packet == &divider_packet1).unwrap() + 1;
    let pos_divider_packet2 = all_packages.iter().position(|packet| packet == &divider_packet2).unwrap() + 1;
    pos_divider_packet1 * pos_divider_packet2
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let package_pairs = parser::parse(&input);
    println!("Part One: {:?}", correctly_ordered_packages_index_sum(&package_pairs));
    println!("Part Two:\n{}", calc_decoder_key(&package_pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn part1_order_works() {
        let package_pairs = parser::parse(EXAMPLE_INPUT);
        assert_eq!(is_correctly_ordered(&package_pairs[0]), true);
        assert_eq!(is_correctly_ordered(&package_pairs[1]), true);
        assert_eq!(is_correctly_ordered(&package_pairs[2]), false);
        assert_eq!(is_correctly_ordered(&package_pairs[3]), true);
        assert_eq!(is_correctly_ordered(&package_pairs[4]), false);
        assert_eq!(is_correctly_ordered(&package_pairs[5]), true);
        assert_eq!(is_correctly_ordered(&package_pairs[6]), false);
        assert_eq!(is_correctly_ordered(&package_pairs[7]), false);
    }

    #[test]
    fn part1_works() {
        let package_pairs = parser::parse(EXAMPLE_INPUT);
        let result = correctly_ordered_packages_index_sum(&package_pairs);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_works() {
        let package_pairs = parser::parse(EXAMPLE_INPUT);
        let result = calc_decoder_key(&package_pairs);
        assert_eq!(result, 140);
    }
}
