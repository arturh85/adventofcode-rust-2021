//!

#[aoc_generator(dayX)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

/// Part 1
#[aoc(dayX, part1)]
fn part1(input: &str) -> usize {
    todo!();
}

/// Part 2
#[aoc(dayX, part2)]
fn part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn part1_examples() {
        assert_eq!(0, part1(&parse_input(EXAMPLE)));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(0, part2(&parse_input(EXAMPLE)));
    }
}
