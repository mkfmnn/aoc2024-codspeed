use std::cmp::Ordering;

static mut RULES: [i8; 0x10000] = [0; 0x10000];

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    #[allow(static_mut_refs)]
    let rules = unsafe { &mut RULES };
    let mut bytes = parse_rules_scalar(bytes, rules);
    let mut pages = Vec::<SortablePage>::with_capacity(24);
    let mut sum = 0;
    while !bytes.is_empty() {
        pages.clear();
        bytes = parse_pages_scalar(bytes, &mut pages);
        if pages.array_windows().all(|[a, b]| a < b) {
            let middle = pages[pages.len() / 2].0 as usize;
            sum += middle;
        }
    }
    sum
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    #[allow(static_mut_refs)]
    let rules = unsafe { &mut RULES };
    let mut bytes = parse_rules_scalar(bytes, rules);
    let mut pages = Vec::<SortablePage>::with_capacity(24);
    let mut sum = 0;
    while !bytes.is_empty() {
        pages.clear();
        bytes = parse_pages_scalar(bytes, &mut pages);
        if pages.array_windows().all(|[a, b]| a < b) {
            continue;
        }
        pages.sort();
        let middle = pages[pages.len() / 2].0 as usize;
        sum += middle;
    }
    sum
}

#[inline]
fn parse_2digit(bytes: &[u8]) -> u8 {
    unsafe {
        bytes
            .get_unchecked(0)
            .wrapping_mul(10)
            .wrapping_add(*bytes.get_unchecked(1))
            .wrapping_sub(16)
    }
}

fn parse_rules_scalar<'a>(bytes: &'a [u8], rules: &mut [i8; 0x10000]) -> &'a [u8] {
    let mut i = 0;
    while bytes[i] != b'\n' {
        unsafe {
            let a = parse_2digit(&bytes[i..i + 2]);
            let b = parse_2digit(&bytes[i + 3..i + 5]);
            *rules.get_unchecked_mut((a as usize) << 8 | b as usize) = -1;
            *rules.get_unchecked_mut((b as usize) << 8 | a as usize) = 1;
        }
        i += 6;
    }
    &bytes[i + 1..]
}

#[derive(Debug)]
struct SortablePage(u8);

impl Ord for SortablePage {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0;
        let b = other.0;
        let o = unsafe {
            #[allow(static_mut_refs)]
            match *RULES.get_unchecked((a as usize) << 8 | b as usize) {
                1 => Ordering::Greater,
                -1 => Ordering::Less,
                _ => unreachable!(),
            }
        };
        o
    }
}

impl PartialOrd for SortablePage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SortablePage {
    fn eq(&self, _other: &Self) -> bool {
        unreachable!()
    }
}

impl Eq for SortablePage {}

fn parse_pages_scalar<'a>(bytes: &'a [u8], pages: &mut Vec<SortablePage>) -> &'a [u8] {
    let mut i = 0;
    loop {
        unsafe {
            let a = parse_2digit(&bytes[i..i + 2]);
            pages.push(SortablePage(a));
            i += 3;
            if *bytes.get_unchecked(i - 1) == b'\n' {
                break;
            }
        }
    }
    &bytes[i..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input5.txt").unwrap();
        assert_eq!(5087, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input5.txt").unwrap();
        assert_eq!(4971, part2(&input));
    }
}
