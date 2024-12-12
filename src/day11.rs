use std::collections::HashMap;

const L: usize = 10_000_000;

static LUT1: [u16; L] = unsafe {
    let mut t = [0u16; L];
    let b = include_bytes!("../lut/d11p1.bin");
    assert!(b.len() == L * 2);
    std::ptr::copy(
        b.as_ptr(),
        std::mem::transmute::<_, &mut [u8; L * 2]>(&mut t).as_mut_ptr(),
        L * 2,
    );
    t
};

static LUT2: [usize; L] = unsafe {
    let mut t = [0usize; L];
    let b = include_bytes!("../lut/d11p2.bin");
    assert!(b.len() == L * 8);
    std::ptr::copy(
        b.as_ptr(),
        std::mem::transmute::<_, &mut [u8; L * 8]>(&mut t).as_mut_ptr(),
        L * 8,
    );
    t
};

pub fn part1(input: &str) -> usize {
    inner(input, |n| unsafe { *LUT1.get_unchecked(n) as usize }, 25)
}

pub fn part2(input: &str) -> usize {
    inner(input, |n| unsafe { *LUT2.get_unchecked(n) }, 75)
}

fn inner<F>(input: &str, lut: F, it: usize) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut cache = HashMap::<(usize, usize), usize>::new();
    let input = input.as_bytes();

    let mut i = 0;
    let mut n = 0;
    let mut r = 0;
    loop {
        let c = unsafe { *input.get_unchecked(i) };
        if c == b' ' || c == b'\n' {
            r += if n < L {
                lut(n)
            } else {
                expand::<100_000>(n, it, &mut cache)
            };
            if c == b'\n' {
                break r;
            }
            n = 0;
        } else {
            n *= 10;
            n += (c - b'0') as usize;
        }
        i += 1;
    }
}

pub fn expand<const C: usize>(
    n: usize,
    it: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if it == 0 {
        return 1;
    }
    if let Some(r) = cache.get(&(n, it)) {
        return *r;
    }
    let r = if n == 0 {
        expand::<C>(1, it - 1, cache)
    } else {
        let digits = n.ilog10() + 1;
        if digits % 2 == 0 {
            let mask = 10usize.pow(digits / 2);
            expand::<C>(n / mask, it - 1, cache) + expand::<C>(n % mask, it - 1, cache)
        } else {
            expand::<C>(n * 2024, it - 1, cache)
        }
    };
    if n < C {
        cache.insert((n, it), r);
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(217443, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(257246536026785, part2(INPUT));
    }
}
