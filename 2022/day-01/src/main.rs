use std::{fs::File, io::Read};

fn sum_elf_calories(input: &str) -> u64 {
    input.trim().split("\n").map(|line| u64::from_str_radix(line, 10).unwrap()).sum()
}

fn calories_sum_top_n(input: &str, top_n: usize) -> u64 {
    let mut calorie_sums: Vec<_> = input.split("\n\n").map(|elf_calories| sum_elf_calories(elf_calories)).collect();
    calorie_sums.sort();
    calorie_sums.iter().rev().take(top_n).sum()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", calories_sum_top_n(&input, 1));
    println!("Part Two: {}", calories_sum_top_n(&input, 3));
}

#[cfg(test)]
mod tests {
    use crate::calories_sum_top_n;

    #[test]
    fn example_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let part_one = calories_sum_top_n(input, 1);
        assert_eq!(part_one, 24000);

        let part_two = calories_sum_top_n(input, 3);
        assert_eq!(part_two, 45000);
    }
}