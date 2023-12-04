use std::fs::read_to_string;

#[derive(Debug)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Draw {
    cubes: Vec<(Cube, usize)>,
}

#[derive(Debug)]
struct Game {
    draws: Vec<Draw>,
}

fn main() {
    let input = read_to_string("./inputs/day-02.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    // println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let games = parser::parse(input);
    dbg!(games);
    todo!()
}

fn solve_part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        let solution = solve_part1(&input);
        assert_eq!(solution, 8);
    }

    //     #[test]
    //     fn part2_works() {
    //         let input = r#"
    // "#;
    //         let solution = solve_part2(&input);
    //         assert_eq!(solution, );
    //     }
}

mod parser {
    use super::*;

    pub(super) fn parse(input: &str) -> Vec<Game> {
        input.lines().map(|l| parse_line(l)).collect()
    }

    fn parse_line(input: &str) -> Game {
        let (_, game_part) = input.split_once(":").unwrap();
        Game {
            draws: parse_draws(game_part.trim()),
        }
    }

    fn parse_draws(input: &str) -> Vec<Draw> {
        input.split(";").map(|d| parse_draw(d.trim())).collect()
    }

    fn parse_draw(input: &str) -> Draw {
        Draw {
            cubes: input.split(",").map(|c| parse_count_and_cube(c.trim())).collect(),
        }
    }

    fn parse_count_and_cube(input: &str) -> (Cube, usize) {
        let (count_str, cube_str) = input.split_once(" ").unwrap();

        let cube = match cube_str {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => panic!("{}", count_str),
        };
        let count = count_str.parse().unwrap();

        (cube, count)
    }
}
