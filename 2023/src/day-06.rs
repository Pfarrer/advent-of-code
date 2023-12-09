fn main() {
    let input: Vec<(usize, usize)> = vec![(47, 282), (70, 1079), (75, 1147), (66, 1062)];
    println!("Solution Part 1: {}", solve_part1(&input));
    println!("Solution Part 2: {}", solve_part2(47707566, 282107911471062));
}

fn solve_part1(input: &Vec<(usize, usize)>) -> usize {
    input
        .iter()
        .cloned()
        .map(|(race_time, distance_to_beat)| count_winning_strategies(race_time, distance_to_beat))
        .fold(1, |acc, v| acc * v)
}

fn solve_part2(race_time: usize, distance_to_beat: usize) -> usize {
    count_winning_strategies(race_time, distance_to_beat)
}

fn count_winning_strategies(race_time: usize, distance_to_beat: usize) -> usize {
    (1..race_time)
        .into_iter()
        .map(|charge_time| race_distance(race_time, charge_time))
        .filter(|distance| *distance > distance_to_beat)
        .count()
}

fn race_distance(race_time: usize, charge_time: usize) -> usize {
    let speed = charge_time;
    let move_time = race_time - charge_time;
    move_time * speed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input: Vec<(usize, usize)> = vec![(7, 9), (15, 40), (30, 200)];
        let solution = solve_part1(&input);
        assert_eq!(solution, 288);
    }

    #[test]
    fn part2_works() {
        let solution = solve_part2(71530, 940200);
        assert_eq!(solution, 71503);
    }
}
