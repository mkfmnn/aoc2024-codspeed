pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(INPUT));
    }
}
