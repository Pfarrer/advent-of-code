use std::{fs::File, io::Read};

#[derive(Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn winning_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn loosing_move(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Won
}

struct Round {
    other: Move,
    myself: Move,
}

impl Round {
    /// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
    /// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    fn score(&self) -> usize {
        let mut score: usize = match self.myself {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        score += match self.outcome() {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        };

        score
    }

    fn outcome(&self) -> Outcome {
        match (&self.other, &self.myself) {
            (Move::Rock, Move::Paper) => Outcome::Won,
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Scissors) => Outcome::Lost,
            (Move::Paper, Move::Rock) => Outcome::Lost,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Won,
            (Move::Scissors, Move::Rock) => Outcome::Won,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
            (Move::Scissors, Move::Paper) => Outcome::Lost,
        }
    }
}

mod part_one {
    use crate::*;


    fn parse_round(input: &str) -> Round {
        let elements: Vec<&str> = input.split_whitespace().collect();

        // First col: A for Rock, B for Paper, and C for Scissors
        let other = match elements[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move: {}", input),
        };

        // Second col: X for Rock, Y for Paper, and Z for Scissors
        let myself = match elements[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move: {}", input),
        };

        Round { other, myself }
    }

    fn parse_rounds(input: &str) -> Vec<Round> {
        input.trim().split("\n").map(|line| parse_round(line)).collect()
    }

    pub fn rock_paper_scissors_score(input: &str) -> usize {
        parse_rounds(input).iter().map(|round| round.score()).sum()
    }
}

mod part_two {
    use crate::*;

    fn parse_round(input: &str) -> Round {
        let elements: Vec<&str> = input.split_whitespace().collect();

        // First col: A for Rock, B for Paper, and C for Scissors
        let other = match elements[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move: {}", input),
        };

        // Second col: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
        let myself = match elements[1] {
            "X" => other.loosing_move(),
            "Y" => other.clone(),
            "Z" => other.winning_move(),
            _ => panic!("Invalid move: {}", input),
        };

        Round { other, myself }
    }

    fn parse_rounds(input: &str) -> Vec<Round> {
        input.trim().split("\n").map(|line| parse_round(line)).collect()
    }

    pub fn rock_paper_scissors_score(input: &str) -> usize {
        parse_rounds(input).iter().map(|round| round.score()).sum()
    }
}

fn main() {
    let mut f = File::open("2022/day-02/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", part_one::rock_paper_scissors_score(&input));
    println!("Part Two: {}", part_two::rock_paper_scissors_score(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_works() {
        let input = "A Y
        B X
        C Z";
        let part_one = part_one::rock_paper_scissors_score(input);
        assert_eq!(part_one, 15);

        let part_two = part_two::rock_paper_scissors_score(input);
        assert_eq!(part_two, 12);
    }
}