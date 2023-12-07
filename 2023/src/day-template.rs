use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./inputs/day-.txt").expect("read input");
    println!("Solution Part 1: {}", solve_part1(&input));
    // println!("Solution Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    todo!()
}

fn solve_part2(input: &str) -> usize {
    todo!()
}

mod parser {
    pub(super) fn parse(input: &str) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = r#"
"#;

    #[test]
    fn part1_works() {
        let solution = solve_part1(&input);
        assert_eq!(solution, );
    }
    
    // #[test]
    // fn part2_works() {
    //     let solution = solve_part2(&input);
    //     assert_eq!(solution, );
    // }
}
