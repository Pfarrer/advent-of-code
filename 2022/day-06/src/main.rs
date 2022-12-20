use std::{collections::VecDeque, fs::File, io::Read};

struct StartOfPacketDetector {
    buf_size: usize,
    char_sequence: VecDeque<char>,
    char_count: [u8; 26],
}

impl StartOfPacketDetector {
    pub fn new(buf_size: usize) -> StartOfPacketDetector {
        StartOfPacketDetector {
            buf_size,
            char_sequence: VecDeque::with_capacity(buf_size),
            char_count: [0; 26],
        }
    }

    pub fn push(&mut self, c: char) {
        if self.char_sequence.len() == self.buf_size {
            let popped_char = self.char_sequence.pop_front().unwrap();
            let popped_index = StartOfPacketDetector::char_to_index(popped_char);
            self.char_count[popped_index] -= 1;
        }

        self.char_sequence.push_back(c);
        let index = StartOfPacketDetector::char_to_index(c);
        self.char_count[index] += 1;
    }

    pub fn marker_found(&self) -> bool {
        self.char_count.iter().filter(|count| **count > 0).count() == self.buf_size
    }

    fn char_to_index(c: char) -> usize {
        ((c as u8) - 97) as usize
    }
}

fn find_start_marker(input: &str, buf_size: usize) -> usize {
    let mut detector = StartOfPacketDetector::new(buf_size);
    for (i, c) in input.chars().enumerate() {
        detector.push(c);
        if detector.marker_found() {
            return i + 1;
        }
    }
    panic!()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", find_start_marker(&input, 4));
    println!("Part Two: {}", find_start_marker(&input, 14));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example1_works() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let part_one = find_start_marker(input, 4);
        assert_eq!(part_one, 7);

        let part_two = find_start_marker(input, 14);
        assert_eq!(part_two, 19);
    }

    #[test]
    fn example2_works() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        let part_one = find_start_marker(input, 4);
        assert_eq!(part_one, 5);

        let part_two = find_start_marker(input, 14);
        assert_eq!(part_two, 23);
    }

    #[test]
    fn example3_works() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";

        let part_one = find_start_marker(input, 4);
        assert_eq!(part_one, 6);

        let part_two = find_start_marker(input, 14);
        assert_eq!(part_two, 23);
    }

    #[test]
    fn example4_works() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        let part_one = find_start_marker(input, 4);
        assert_eq!(part_one, 10);

        let part_two = find_start_marker(input, 14);
        assert_eq!(part_two, 29);
    }

    #[test]
    fn example5_works() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let part_one = find_start_marker(input, 4);
        assert_eq!(part_one, 11);

        let part_two = find_start_marker(input, 14);
        assert_eq!(part_two, 26);
    }
}
