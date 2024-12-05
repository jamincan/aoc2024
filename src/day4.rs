use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use bstr::ByteSlice;

struct Puzzle {
    letters: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl Puzzle {
    fn check_with_offset(&self, word: &[u8], index: usize, offset: i64) -> bool {
        let letters = (0..).map(|i| self.letters[(index as i64 + (i * offset)) as usize]);
        for (a, b) in word.iter().zip(letters) {
            if *a != b {
                return false;
            }
        }
        true
    }

    fn check_xmas_at(&self, index: usize) -> usize {
        let mut matches = 0;
        for offset in self.xmas_offsets_at(index) {
            if self.check_with_offset(b"XMAS", index, offset) {
                matches += 1;
            }
        }
        matches
    }

    fn xmas_offsets_at(&self, index: usize) -> impl Iterator<Item = i64> {
        let column = index % self.columns;
        let row = index / self.columns;
        let space_above = row + 1 >= 4;
        let space_below = self.rows - row >= 4;
        let space_left = column + 1 >= 4;
        let space_right = self.columns - column >= 4;

        let constrained_offsets = [
            (space_above, -(self.columns as i64)),
            (space_above && space_left, -(self.columns as i64) - 1),
            (space_above && space_right, -(self.columns as i64) + 1),
            (space_left, -1),
            (space_right, 1),
            (space_below, (self.columns as i64)),
            (space_below && space_left, (self.columns as i64) - 1),
            (space_below && space_right, (self.columns as i64) + 1),
        ];

        constrained_offsets
            .into_iter()
            .filter(|(valid, _)| *valid)
            .map(|(_, offset)| offset)
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &[u8]) -> Result<Puzzle> {
    // Remove new lines and replace with three spaces after every line. This simplifies tests so that
    // they can be done with a simple offset and don't wrap around to the next row when doing so.
    let letters: Vec<_> = input.lines().flatten().copied().collect();
    let columns = input.lines().next().context("no end of line found")?.len();
    let rows = letters.len() / columns;
    Ok(Puzzle {
        letters,
        columns,
        rows,
    })
}

#[aoc(day4, part1)]
fn solve_part1(puzzle: &Puzzle) -> usize {
    (0..puzzle.letters.len())
        .map(|index| puzzle.check_xmas_at(index))
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(puzzle: &Puzzle) -> usize {
    let start = puzzle.columns + 1; // Start on R1C1
    let end = puzzle.letters.len() - puzzle.columns - 1; // End on second last row, second last column
    let indices = (start..=end)
        .filter(|index| index % puzzle.columns > 0 && index % puzzle.columns < puzzle.columns - 1); // Filter out first and last column
                                                                                                    // let mut write = vec![b'.'; puzzle.letters.len()];

    let mut matches = 0;
    for index in indices {
        if puzzle.letters[index] != b'A' {
            continue;
        }
        let first = (
            puzzle.letters[index - puzzle.columns - 1],
            puzzle.letters[index + puzzle.columns + 1],
        );
        let second = (
            puzzle.letters[index - puzzle.columns + 1],
            puzzle.letters[index + puzzle.columns - 1],
        );
        if (first == (b'M', b'S') || first == (b'S', b'M'))
            && (second == (b'M', b'S') || second == (b'S', b'M'))
        {
            matches += 1;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[u8] = b"\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn check_offset_at() {
        let puzzle = parse_input(TEST_INPUT).unwrap();
        let expected = [
            (0, vec![1, 10, 11]),
            (5, vec![-1, 1, 9, 10, 11]),
            (9, vec![-1, 9, 10]),
            (50, vec![-10, -9, 1, 10, 11]),
            (55, vec![-11, -10, -9, -1, 1, 9, 10, 11]),
            (59, vec![-11, -10, -1, 9, 10]),
            (90, vec![-10, -9, 1]),
            (95, vec![-11, -10, -9, -1, 1]),
            (99, vec![-11, -10, -1]),
        ];
        for (index, offsets) in expected {
            let mut result: Vec<_> = puzzle.xmas_offsets_at(index).collect();
            result.sort();
            assert_eq!(result, offsets);
        }
    }

    #[test]
    fn pt1_count_xmases() {
        let puzzle = parse_input(TEST_INPUT).unwrap();
        let result = solve_part1(&puzzle);
        assert_eq!(result, 18);
    }

    #[test]
    fn pt2_count_x_mases() {
        let puzzle = parse_input(TEST_INPUT).unwrap();
        let result = solve_part2(&puzzle);
        assert_eq!(result, 9);
    }
}
