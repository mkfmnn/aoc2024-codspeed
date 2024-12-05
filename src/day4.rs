const DIM: usize = 140;

const fn step(d: u8) -> isize {
    const IDIM: isize = DIM as isize;
    match d {
        0 => 1,         // E
        1 => IDIM + 2,  // SE
        2 => IDIM + 1,  // S
        3 => IDIM,      // SW
        4 => -1,        // W
        5 => -IDIM - 2, // NW
        6 => -IDIM - 1, // N
        7 => -IDIM,     // NE
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();
    assert_eq!(DIM * (DIM + 1), bytes.len());
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'X' {
            sum += check_dir::<0>(bytes, i)
                + check_dir::<1>(bytes, i)
                + check_dir::<2>(bytes, i)
                + check_dir::<3>(bytes, i)
                + check_dir::<4>(bytes, i)
                + check_dir::<5>(bytes, i)
                + check_dir::<6>(bytes, i)
                + check_dir::<7>(bytes, i);
        }
    }
    sum
}

fn check_dir<const D: u8>(bytes: &[u8], i: usize) -> usize {
    let step = step(D) as usize;
    unsafe {
        if bytes.get(i.overflowing_add(step.overflowing_mul(3).0).0) == Some(&b'S')
        && *bytes.get_unchecked(i.overflowing_add(step.overflowing_mul(2).0).0) == b'A'
        && *bytes.get_unchecked(i.overflowing_add(step).0) == b'M'
        {
            1
        } else {
            0
        }
    }
}

pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut sum = 0;
    for y in 1..DIM - 1 {
        let off = y * (DIM + 1);
        for x in 1..DIM - 1 {
            unsafe {
                if *bytes.get_unchecked(off + x) == b'A'
                    && *bytes.get_unchecked(off + x + DIM + 2)
                        ^ *bytes.get_unchecked(off + x - DIM - 2)
                        == 30
                    && *bytes.get_unchecked(off + x + DIM) ^ *bytes.get_unchecked(off + x - DIM)
                        == 30
                {
                    sum += 1
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2549, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2003, part2(&input));
    }
}
