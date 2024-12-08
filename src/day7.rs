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

macro_rules! recurse {
    () => {
        recurse! {recurse1, recurse1_u32, false}
        recurse! {recurse2, recurse2_u32, true}
    };
    ( $fn_64:ident, $fn_32:ident, $concat:expr ) => {
        recurse! {$fn_64, $fn_32, $concat, u64, true}
        recurse! {$fn_32, $fn_32, $concat, u32, false}
    };
    ( $my_fn:ident, $fn_32:ident, $concat:expr, $type:ty, $is_64:expr) => {
        fn $my_fn(target: $type, idx: usize, nums: &[u32]) -> bool {
            if $is_64 && target <= u32::MAX as $type {
                return $fn_32(target as u32, idx, nums);
            }
            let last = unsafe { *nums.get_unchecked(idx) } as $type;
            if idx == 0 {
                return last == target;
            }
            if target % last == 0 && $my_fn(target / last, idx - 1, nums) {
                return true;
            }
            if $concat {
                let last_mul = unsafe {
                    match last {
                        0..10 => 10,
                        10..100 => 100,
                        100..1000 => 1000,
                        _ => std::hint::unreachable_unchecked(),
                    }
                };
                if target % last_mul == last && $my_fn(target / last_mul, idx - 1, nums) {
                    return true;
                }
            }
            target > last && $my_fn(target - last, idx - 1, nums)
        }
    };
}

recurse! {}

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
