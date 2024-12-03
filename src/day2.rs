use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    levels: Vec<i32>,
}

impl std::str::FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values = input.split_whitespace();
        let levels = values.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Report { levels })
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Result<Vec<Report>> {
    input.trim().lines().map(str::parse).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(reports: &[Report]) -> usize {
    let safe_count = reports
        .iter()
        .filter(|report| is_safe(&report.levels))
        .count();
    safe_count
}

fn is_safe<'i, T: IntoIterator<Item = &'i i32>>(levels: T) -> bool {
    use itertools::Itertools;
    let mut levels_iter = levels.into_iter().tuple_windows().peekable();
    let Some((first, second)) = levels_iter.peek() else {
        return true;
    };
    let is_increasing = second > first;
    levels_iter.all(|(first, second)| {
        let diff = second.abs_diff(*first);
        (1..=3).contains(&diff) && (second > first) == is_increasing
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(reports: &[Report]) -> usize {
    let safe_count = reports
        .iter()
        .filter(|report| dampened_is_safe(&report.levels))
        .count();
    safe_count
}

fn dampened_is_safe(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for skip in 0..levels.len() {
        let levels_iter = levels
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != skip)
            .map(|(_, level)| level);
        if is_safe(levels_iter) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
                              1 2 7 8 9
                              9 7 6 2 1
                              1 3 2 4 5
                              8 6 4 4 1
                              1 3 6 7 9";

    #[test]
    fn parsing() {
        let result = parse_input(TEST_INPUT).unwrap();
        let expected = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn pt1_safety() {
        let reports = parse_input(TEST_INPUT).unwrap();
        let pt1_safety_results = [true, false, false, false, false, true];

        for (report, safety) in reports.iter().zip(pt1_safety_results) {
            assert_eq!(is_safe(&report.levels), safety);
        }
    }

    #[test]
    fn pt2_safety() {
        let reports = parse_input(TEST_INPUT).unwrap();
        let pt2_safety_results = [true, false, false, true, true, true];

        for (report, safety) in reports.iter().zip(pt2_safety_results) {
            assert_eq!(dampened_is_safe(&report.levels), safety);
        }
    }

    #[test]
    fn pt1_safe_count() {
        let input = parse_input(TEST_INPUT).unwrap();
        let result = solve_part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn pt2_safe_count() {
        let input = parse_input(TEST_INPUT).unwrap();
        let result = solve_part2(&input);
        assert_eq!(result, 4);
    }
}
