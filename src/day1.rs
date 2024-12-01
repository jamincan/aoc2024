use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut ids = line.split_whitespace();
        left.push(ids.next().and_then(|id| str::parse(id).ok()).expect("first id should be an integer"));
        right.push(ids.next().and_then(|id| str::parse(id).ok()).expect("second id should be an integer"));
    }
    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<i32>, Vec<i32>)) -> u32 {
    let mut left = input.0.clone();
    let mut right = input.1.clone();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut right = FxHashMap::default();
    for id in input.1.iter() {
        right.entry(id).and_modify(|count| *count += 1).or_insert(1);
    }
    input.0.iter().map(|id| id * right.get(id).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
                              4   3
                              2   5
                              1   3
                              3   9
                              3   3";

    #[test]
    fn parsing() {
        let (left, right) = parse_input(TEST_INPUT);
        assert_eq!(left, vec![3,4,2,1,3,3]);
        assert_eq!(right, vec![4,3,5,3,9,3]);
    }

    #[test]
    fn part1_find_sum_of_differences() {
        let input = parse_input(TEST_INPUT);
        let result = solve_part1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2_calculate_similarity_score() {
        let input = parse_input(TEST_INPUT);
        let result = solve_part2(&input);
        assert_eq!(result, 31);
    }
}