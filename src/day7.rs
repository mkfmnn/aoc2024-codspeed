pub fn part1(input: &str) -> u64 {
    unsafe { part_inner(input.as_bytes(), recurse1) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { part_inner(input.as_bytes(), recurse2) }
}

unsafe fn part_inner<F>(mut input: &[u8], f: F) -> u64
where
    F: Fn(u64, usize, &[u32]) -> bool,
{
    let mut sum = 0;
    let mut nums = [0u32; 20];
    while !input.is_empty() {
        let (target, next_input) = parse_target_fast(input);
        let mut i = 0;
        input = next_input;
        loop {
            let (n, eol, next_input) = parse_fast(input);
            *nums.get_unchecked_mut(i) = n;
            i += 1;
            input = next_input;
            if eol {
                break;
            }
        }
        if f(target, i - 1, &nums) {
            sum += target;
        }
    }
    sum
}

unsafe fn parse_target_fast(input: &[u8]) -> (u64, &[u8]) {
    let mut n = *input.get_unchecked(0) as u64 - b'0' as u64;
    let mut i = 1;
    loop {
        let c = *input.get_unchecked(i);
        if c == b':' {
            return (n, input.get_unchecked(i + 2..));
        }
        n *= 10;
        n += c as u64 - b'0' as u64;
        i += 1;
    }
}

unsafe fn parse_fast(input: &[u8]) -> (u32, bool, &[u8]) {
    let c1 = *input.get_unchecked(0);
    let c2 = *input.get_unchecked(1);
    if c2 == b' ' || c2 == b'\n' {
        return (
            c1 as u32 - b'0' as u32,
            c2 == b'\n',
            input.get_unchecked(2..),
        );
    }
    let c3 = *input.get_unchecked(2);
    if c3 == b' ' || c3 == b'\n' {
        return (
            c1 as u32 * 10 + c2 as u32 - b'0' as u32 * 11,
            c3 == b'\n',
            input.get_unchecked(3..),
        );
    };
    let c4 = *input.get_unchecked(3);
    if c4 == b' ' || c4 == b'\n' {
        return (
            c1 as u32 * 100 + c2 as u32 * 10 + c3 as u32 - b'0' as u32 * 111,
            c4 == b'\n',
            input.get_unchecked(4..),
        );
    };
    std::hint::unreachable_unchecked();
}

fn fast_logpow(n: u64) -> u64 {
    unsafe {
        match n {
            0..10 => 10,
            10..100 => 100,
            100..1000 => 1000,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}

fn recurse1(target: u64, idx: usize, nums: &[u32]) -> bool {
    if target <= u32::MAX as u64 {
        return recurse1_u32(target as u32, idx, nums);
    }
    let last = unsafe { *nums.get_unchecked(idx) } as u64;
    if idx == 0 {
        return last == target;
    }
    if target % last == 0 && recurse1(target / last, idx - 1, nums) {
        return true;
    }
    target > last && recurse1(target - last, idx - 1, nums)
}

fn recurse1_u32(target: u32, idx: usize, nums: &[u32]) -> bool {
    let last = unsafe { *nums.get_unchecked(idx) };
    if idx == 0 {
        return last == target;
    }
    if target % last == 0 && recurse1_u32(target / last, idx - 1, nums) {
        return true;
    }
    target > last && recurse1_u32(target - last, idx - 1, nums)
}

fn recurse2(target: u64, idx: usize, nums: &[u32]) -> bool {
    if target <= u32::MAX as u64 {
        return recurse2_u32(target as u32, idx, nums);
    }
    let last = unsafe { *nums.get_unchecked(idx) } as u64;
    if idx == 0 {
        return last == target;
    }
    if target % last == 0 && recurse2(target / last, idx - 1, nums) {
        return true;
    }
    let last_mul = fast_logpow(last);
    if target % last_mul == last && recurse2(target / last_mul, idx - 1, nums) {
        return true;
    }
    target > last && recurse2(target - last, idx - 1, nums)
}

fn recurse2_u32(target: u32, idx: usize, nums: &[u32]) -> bool {
    let last = unsafe { *nums.get_unchecked(idx) };
    if idx == 0 {
        return last == target;
    }
    if target % last == 0 && recurse2_u32(target / last, idx - 1, nums) {
        return true;
    }
    let last_mul = fast_logpow(last as u64) as u32;
    if target % last_mul == last && recurse2_u32(target / last_mul, idx - 1, nums) {
        return true;
    }
    target > last && recurse2_u32(target - last, idx - 1, nums)
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
