use std::fs::read_to_string;

struct Card {
    winning_numbers: Vec<usize>,
    listed_numbers: Vec<usize>,
    copies: usize,
}

fn main() {
    let input = read_to_string("./inputs/day-04.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let cards = parser::parse(input);
    cards.iter().map(|card| card_value(card)).sum()
}

fn solve_part2(input: &str) -> usize {
    let mut cards = parser::parse(input);
    update_card_copies(&mut cards);

    cards.iter().map(|card| card.copies).sum()
}

fn update_card_copies(cards: &mut Vec<Card>) {
    for i in 0..cards.len() {
        let matching_numbers_count = matching_numbers_count(&cards[i]);
        increment_card_copies(cards, i, matching_numbers_count);
    }
}

fn increment_card_copies(cards: &mut Vec<Card>, start_exclusive: usize, count: usize) {
    if count == 0 || cards.len() == start_exclusive + 1 {
        return;
    }

    let increment = cards[start_exclusive].copies;

    let start = std::cmp::min(cards.len() - 1, start_exclusive + 1);
    let end = std::cmp::min(cards.len(), start + count);

    cards[start..end]
        .iter_mut()
        .for_each(|card| card.copies += increment);
}

fn matching_numbers_count(card: &Card) -> usize {
    let both_numbers = {
        let mut n = card.listed_numbers.clone();
        n.extend(card.winning_numbers.iter());
        n.sort();
        n.dedup();
        n
    };

    (card.winning_numbers.len() + card.listed_numbers.len()) - both_numbers.len()
}

fn card_value(card: &Card) -> usize {
    let matching_num_count = matching_numbers_count(card);
    if matching_num_count == 0 {
        0
    } else {
        2usize.pow(matching_num_count as u32 - 1)
    }
}

mod parser {
    use super::*;

    pub(super) fn parse(input: &str) -> Vec<Card> {
        input.lines().map(|line| parse_card(line)).collect()
    }

    fn parse_card(line: &str) -> Card {
        let (_, numbers_str) = line.split_once(": ").unwrap();
        let (winning_nums_str, listed_nums_str) = numbers_str.split_once(" | ").unwrap();
        Card {
            winning_numbers: parse_numbers(winning_nums_str),
            listed_numbers: parse_numbers(listed_nums_str),
            copies: 1,
        }
    }

    fn parse_numbers(numbers_str: &str) -> Vec<usize> {
        numbers_str
            .split(' ')
            .flat_map(|num_str| num_str.parse().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn part2_works() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let solution = solve_part2(&input);
        assert_eq!(solution, 30);
    }

    #[test]
    fn increment_card_copies_works() {
        fn run_test(start_exclusive: usize, count: usize) -> Vec<usize> {
            let mut cards = vec![
                Card {
                    listed_numbers: vec![],
                    winning_numbers: vec![],
                    copies: 1,
                },
                Card {
                    listed_numbers: vec![],
                    winning_numbers: vec![],
                    copies: 1,
                },
                Card {
                    listed_numbers: vec![],
                    winning_numbers: vec![],
                    copies: 1,
                },
            ];

            increment_card_copies(&mut cards, start_exclusive, count);

            cards.iter().map(|c| c.copies).collect_vec()
        }

        assert_eq!(run_test(0, 0), vec![1, 1, 1]);
        assert_eq!(run_test(1, 0), vec![1, 1, 1]);
        assert_eq!(run_test(2, 0), vec![1, 1, 1]);

        assert_eq!(run_test(0, 1), vec![1, 2, 1]);
        assert_eq!(run_test(1, 1), vec![1, 1, 2]);
        assert_eq!(run_test(2, 1), vec![1, 1, 1]);

        assert_eq!(run_test(0, 2), vec![1, 2, 2]);
        assert_eq!(run_test(1, 2), vec![1, 1, 2]);
        assert_eq!(run_test(2, 2), vec![1, 1, 1]);
    }
}
