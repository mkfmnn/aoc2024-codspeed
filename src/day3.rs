use core::str;

use memchr::{memchr, memchr2};

pub fn part1(input: &str) -> i64 {
    let mut haystack = input.as_bytes();
    let mut sum = 0;
    while !haystack.is_empty() {
        let Some(location) = memchr(b'u', haystack) else {
            break;
        };
        if let Some(token) = haystack.get(location - 1..location + 3) {
            if token == b"mul(" {
                if let Some((l, r, len)) = parse_mul_args(&haystack[location + 3..]) {
                    sum += l * r;
                    haystack = &haystack[location + 3 + len..];
                    continue;
                }
            }
        }
        haystack = &haystack[location + 1..];
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let mut haystack = input.as_bytes();
    let mut enabled = true;
    let mut sum = 0;
    while !haystack.is_empty() {
        let Some(location) = memchr2(b'u', b'd', haystack) else {
            break;
        };
        if haystack[location] == b'u' {
            if enabled {
                if let Some(token) = haystack.get(location - 1..location + 3) {
                    if token == b"mul(" {
                        if let Some((l, r, len)) = parse_mul_args(&haystack[location + 3..]) {
                            sum += l * r;
                            haystack = &haystack[location + 3 + len..];
                            continue;
                        }
                    }
                }
            }
            haystack = &haystack[location + 1..];
        } else if haystack[location] == b'd' {
            haystack = &haystack[location + 1..];
            if haystack.starts_with(b"o()") {
                enabled = true;
                haystack = &haystack[3..];
            } else if haystack.starts_with(b"on't()") {
                enabled = false;
                haystack = &haystack[5..];
            }
        } else {
            unreachable!();
        }
    }
    sum
}

pub fn parse_mul_args(slice: &[u8]) -> Option<(i64, i64, usize)> {
    if slice.len() < 4 {
        return None;
    }
    let mut n1 = slice[0] as i64 - b'0' as i64;
    if n1 < 0 || n1 >= 10 {
        return None;
    }
    let mut i = 1;
    while i < slice.len() && slice[i].is_ascii_digit() {
        n1 *= 10;
        n1 += slice[i] as i64 - b'0' as i64;
        i += 1;
    }
    if slice.len() - i < 3 || slice[i] != b',' {
        return None;
    }
    let mut n2 = slice[i + 1] as i64 - b'0' as i64;
    if n2 < 0 || n2 >= 10 {
        return None;
    }
    i += 2;
    while i < slice.len() && slice[i].is_ascii_digit() {
        n2 *= 10;
        n2 += slice[i] as i64 - b'0' as i64;
        i += 1;
    }
    if slice.get(i) != Some(&b')') {
        return None;
    }
    Some((n1, n2, i + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        assert_eq!(Some((1, 2, 4)), parse_mul_args(b"1,2)"));
        assert_eq!(Some((100, 999, 8)), parse_mul_args(b"100,999)"));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input3.txt").unwrap();
        assert_eq!(183788984, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input3.txt").unwrap();
        assert_eq!(62098619, part2(&input));
    }
}
