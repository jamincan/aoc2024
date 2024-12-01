use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().map(|line| {
        let mut ids = line.split_whitespace();
        let left: i32 = ids.next().and_then(|id| str::parse(id).ok()).expect("first id should be an integer");
        let right: i32 = ids.next().and_then(|id| str::parse(id).ok()).expect("second id should be an integer");
        (left, right)
    }).unzip()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<i32>, Vec<i32>)) -> u32 {
    let (mut left, mut right) = input.clone();
    left.sort_unstable();
    right.sort_unstable();
    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut right_map = FxHashMap::default();
    for id in right.iter() {
        *right_map.entry(id).or_default() += 1;
    }
    left.iter().map(|id| id * right_map.get(id).unwrap_or(&0)).sum()
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