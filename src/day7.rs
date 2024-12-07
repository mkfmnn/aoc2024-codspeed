pub fn part1(input: &str) -> u64 {
    part_inner(input.as_bytes(), recurse1)
}

pub fn part2(input: &str) -> u64 {
    part_inner(input.as_bytes(), recurse2)
}

fn part_inner<F>(mut input: &[u8], f: F) -> u64
where
    F: Fn(u64, &[u64]) -> bool,
{
    let mut sum = 0;
    let mut nums = Vec::new();
    while !input.is_empty() {
        let (target, i) = atoi_simd::parse_any_pos(input).unwrap();
        input = &input[i + 2..];
        loop {
            let (n, eol, next_input) = unsafe { parse_fast(input) };
            nums.push(n);
            input = next_input;
            if eol {
                break;
            }
        }
        if f(target, &nums) {
            sum += target;
        }
        nums.clear();
    }
    sum
}

unsafe fn parse_fast(input: &[u8]) -> (u64, bool, &[u8]) {
    let c1 = *input.get_unchecked(0);
    let c2 = *input.get_unchecked(1);
    if c2 == b' ' || c2 == b'\n' {
        return (
            c1 as u64 - b'0' as u64,
            c2 == b'\n',
            input.get_unchecked(2..),
        );
    }
    let c3 = *input.get_unchecked(2);
    if c3 == b' ' || c3 == b'\n' {
        return (
            c1 as u64 * 10 + c2 as u64 - b'0' as u64 * 11,
            c3 == b'\n',
            input.get_unchecked(3..),
        );
    };
    let c4 = *input.get_unchecked(3);
    if c4 == b' ' || c4 == b'\n' {
        return (
            c1 as u64 * 100 + c2 as u64 * 10 + c3 as u64 - b'0' as u64 * 111,
            c4 == b'\n',
            input.get_unchecked(4..),
        );
    };
    unreachable!();
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
