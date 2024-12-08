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
    const INPUT: &str = include_str!("../data/input8.txt");

    #[test]
    fn test_part1() {
        assert_eq!(379, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1339, part2(INPUT));
    }
}
