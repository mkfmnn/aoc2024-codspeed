pub fn part1(input: &str) -> u64 {
    part_inner(input, recurse1)
}

pub fn part2(input: &str) -> u64 {
    part_inner(input, recurse2)
}

pub fn part_inner<F>(input: &str, f: F) -> u64
where
    F: Fn(u64, &[u64]) -> bool,
{
    let mut sum = 0;
    for line in input.lines() {
        let (target, rest) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = rest
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        if f(target, &nums) {
            sum += target;
        }
    }
    sum
}

fn recurse1(target: u64, nums: &[u64]) -> bool {
    let (&last, rest) = nums.split_last().unwrap();
    if rest.is_empty() {
        return last == target;
    }
    if target % last == 0 && recurse1(target / last, rest) {
        return true;
    }
    target > last && recurse1(target - last, rest)
}

fn recurse2(target: u64, nums: &[u64]) -> bool {
    let (&last, rest) = nums.split_last().unwrap();
    if rest.is_empty() {
        return last == target;
    }
    if target % last == 0 && recurse2(target / last, rest) {
        return true;
    }
    let last_digits = last.ilog10() + 1;
    let last_mul = 10u64.pow(last_digits);
    if target % last_mul == last && recurse2(target / last_mul, rest) {
        return true;
    }
    target > last && recurse2(target - last, rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(4555081946288, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(227921760109726, part2(INPUT));
    }
}
