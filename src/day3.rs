use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> Option<(i32, i32, &str)> {
    if !input.starts_with("mul(") {
        return None;
    };
    let remainder = &input[4..];
    let (args, remainder) = remainder.split_once(')')?;
    let (x, y) = args.split_once(',')?;
    let x = x.parse().ok()?;
    let y = y.parse().ok()?;
    Some((x, y, remainder))
}

#[aoc_generator(day3)]
pub fn parse_input(mut input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    while input.len() > 0 {
        if input.starts_with("do()") {
            instructions.push(Instruction::Do);
            input = &input[4..];
        } else if input.starts_with("don't()") {
            instructions.push(Instruction::Dont);
            input = &input[7..];
        } else if let Some((x, y, remainder)) = parse_mul(input) {
            instructions.push(Instruction::Mul(x, y));
            input = remainder;
        } else {
            // No matches skip to the next character in the string
            let mut chars = input.chars();
            chars.next();
            input = chars.as_str();
        }
    }
    instructions
}

#[aoc(day3, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(x, y) => Some(x * y),
            _ => None,
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> i32 {
    let mut do_multiply = true;
    instructions
        .iter()
        .filter_map(|instruction| {
            match instruction {
                Instruction::Mul(x, y) if do_multiply => return Some(x * y),
                Instruction::Do => do_multiply = true,
                Instruction::Dont => do_multiply = false,
                _ => (),
            };
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PT1_TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const PT2_TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parsing() {
        use Instruction::*;
        let result = parse_input(PT1_TEST_INPUT);
        let expected = vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)];
        assert_eq!(result, expected);

        let result = parse_input(PT2_TEST_INPUT);
        let expected = vec![Mul(2, 4), Dont, Mul(5, 5), Mul(11, 8), Do, Mul(8, 5)];
        assert_eq!(result, expected);
    }

    #[test]
    fn pt1_multiply() {
        let result = solve_part1(&parse_input(PT1_TEST_INPUT));
        assert_eq!(result, 161);
    }

    #[test]
    fn pt2_multiply() {
        let result = solve_part2(&parse_input(PT2_TEST_INPUT));
        assert_eq!(result, 48);
    }
}
