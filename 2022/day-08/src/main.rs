use std::{fs::File, io::Read, cmp::max};

type Forrest = Vec<TreeRow>;
type TreeRow = Vec<u8>;

fn vertical_iterator<'a>(forrest: &'a Forrest, column: usize, top_to_bottom: bool) -> Box<dyn Iterator<Item=&'a u8> + 'a> {
    if top_to_bottom {
        Box::new((0..forrest[0].len()).into_iter().map(move |row| &forrest[row][column]))
    } else {
        Box::new((0..forrest[0].len()).rev().into_iter().map(move |row| &forrest[row][column]))
    }
}

fn visible_tree_indexes(tree_row_iter: impl Iterator<Item=u8>) -> Vec<usize> {
    let mut visbible_indexes = vec![];

    let mut higest_tree: u8 = 0;
    for (index, tree_heigth) in tree_row_iter.enumerate() {
        if tree_heigth > higest_tree || index == 0 {
            higest_tree = tree_heigth;
            visbible_indexes.push(index);
        }
    }

    visbible_indexes
}

fn count_visible_trees(input: &str) -> usize {
    let forrest = read_forrest(input);
    let mut visible_tree_positions: Vec<(usize, usize)> = Vec::new();
    
    let rows_count = forrest.len();
    let cols_count = forrest[0].len();

    println!("Processing forrest with {rows_count} rows and {cols_count} columns...");

    // Horizontal first
    for row in 0..rows_count {        
        let mut ltr: Vec<(usize, usize)> = visible_tree_indexes(forrest[row].iter().cloned()).iter().map(|index| (*index, row)).collect();
        visible_tree_positions.append(&mut ltr);
        
        let mut rtl: Vec<(usize, usize)> = visible_tree_indexes(forrest[row].iter().rev().cloned()).iter().map(|index| (cols_count-*index-1, row)).collect();
        visible_tree_positions.append(&mut rtl);
    }

    // Vertical second
    for column in 0..cols_count {
        {let ttb_iter = vertical_iterator(&forrest, column, true).cloned();
        let mut ttb: Vec<(usize, usize)> = visible_tree_indexes(ttb_iter).iter().map(|index| (column, *index)).collect();
        visible_tree_positions.append(&mut ttb);}

        let btt_iter = vertical_iterator(&forrest, column, false).cloned();
        let mut btt: Vec<(usize, usize)> = visible_tree_indexes(btt_iter).iter().map(|index| (column, rows_count-*index-1)).collect();
        visible_tree_positions.append(&mut btt);
    }

    visible_tree_positions.sort();
    visible_tree_positions.dedup();

    visible_tree_positions.len()
}

fn make_iter_from(forrest: &Forrest, start: (usize, usize), direction: (i32, i32)) -> impl Iterator<Item=u8> + '_ {
    struct ForrestIter<'a> {
        forrest: &'a Forrest,
        pos: (i32, i32),
        direction: (i32, i32),
    }

    impl<'a> Iterator for ForrestIter<'a> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            self.pos.0 += self.direction.0;
            self.pos.1 += self.direction.1;

            if self.pos.0 < 0 || self.pos.1 < 0 {
                return None;
            }
            
            self.forrest.get(max(0, self.pos.0) as usize).and_then(|tree_row|
                tree_row.get(max(0, self.pos.1) as usize).copied()
            )
        }
    }

    ForrestIter {
        forrest,
        pos: (start.0 as i32, start.1 as i32),
        direction,
    }
}

fn calc_scenic_score(forrest: &Forrest, row: usize, col: usize) -> usize {
    let rows_count = forrest.len();
    let cols_count = forrest[0].len();
    let tree_hight = forrest[row][col];


    let score_right = make_iter_from(forrest, (row, col), (0, 1))
        .position(|candidate_hight| candidate_hight >= tree_hight)
        .map_or_else(|| cols_count - col - 1, |pos| pos + 1);

    let score_left = make_iter_from(forrest, (row, col), (0, -1))
        .position(|candidate_hight| candidate_hight >= tree_hight)
        .map_or_else(|| col, |pos| pos + 1);
    
    let score_top = make_iter_from(forrest, (row, col), (-1, 0))
        .position(|candidate_hight| candidate_hight >= tree_hight)
        .map_or_else(|| row, |pos| pos + 1);
    

    let score_bottom = make_iter_from(forrest, (row, col), (1, 0))
        .position(|candidate_hight| candidate_hight >= tree_hight)
        .map_or_else(|| rows_count - row - 1, |pos| pos + 1);
    
    score_right * score_left * score_top * score_bottom
}

fn highest_scenic_score(input: &str) -> usize {
    let forrest = read_forrest(input);
    
    let rows_count = forrest.len();
    let cols_count = forrest[0].len();

    let mut max_score: usize = 0;
    for row in 0..rows_count {
        for col in 0..cols_count {
            let score = calc_scenic_score(&forrest, row, col);
            max_score = max(score, max_score);
        }
    }

    max_score
}

fn read_tree_row(input: &str) -> TreeRow {
    input.trim().chars().map(|c| (c as u8) - 48).collect()
}

fn read_forrest(input: &str) -> Forrest {
    input.trim().split("\n").map(|line| read_tree_row(line)).collect()
}

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("Part One: {}", count_visible_trees(&input));
    println!("Part Two: {}", highest_scenic_score(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn vertical_iterator_works() {
        let forrest = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        let mut iter_ttb = vertical_iterator(&forrest, 1, true);
        assert_eq!(iter_ttb.next(), Some(&2));
        assert_eq!(iter_ttb.next(), Some(&5));
        assert_eq!(iter_ttb.next(), Some(&8));
        assert_eq!(iter_ttb.next(), None);

        let mut iter_btt = vertical_iterator(&forrest, 2, false);
        assert_eq!(iter_btt.next(), Some(&9));
        assert_eq!(iter_btt.next(), Some(&6));
        assert_eq!(iter_btt.next(), Some(&3));
        assert_eq!(iter_btt.next(), None);
    }

    
    #[test]
    fn calc_scenic_score_works() {
        let vertical_forrest = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 2, 1, 3, 2, 4, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];

        assert_eq!(calc_scenic_score(&vertical_forrest, 1, 1), 2);
        assert_eq!(calc_scenic_score(&vertical_forrest, 1, 5), 5);
        assert_eq!(calc_scenic_score(&vertical_forrest, 1, 0), 0);
        assert_eq!(calc_scenic_score(&vertical_forrest, 1, 6), 0);


        let horizontal_forrest = vec![
            vec![1, 1, 1],
            vec![1, 3, 1],
            vec![1, 7, 1],
            vec![1, 6, 1],
            vec![1, 1, 1],
        ];

        assert_eq!(calc_scenic_score(&horizontal_forrest, 1, 1), 1);
        assert_eq!(calc_scenic_score(&horizontal_forrest, 2, 1), 4);
        assert_eq!(calc_scenic_score(&horizontal_forrest, 0, 1), 0);
        assert_eq!(calc_scenic_score(&horizontal_forrest, 4, 1), 0);
    }

    #[test]
    fn example_works() {
        let input = "30373
25512
65332
33549
35390";

        let part_one = count_visible_trees(input);
        assert_eq!(part_one, 21);

        let part_two = highest_scenic_score(input);
        assert_eq!(part_two, 8);
    }
}