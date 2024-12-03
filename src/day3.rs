use core::str;
use std::sync::LazyLock;

use regex::bytes::Regex;

static RE_1: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap());

static RE_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]+),([0-9]+)\)").unwrap());

pub fn part1(input: &str) -> i64 {
    RE_1.captures_iter(input.as_bytes())
        .map(|c| {
            let [l, r] = c.extract().1;
            fastparse(l) * fastparse(r)
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut enabled = true;
    let mut sum = 0;
    for capture in RE_2.captures_iter(input.as_bytes()) {
        if capture.get(1).is_some() {
            enabled = true;
        } else if capture.get(2).is_some() {
            enabled = false;
        } else if enabled {
            let l = fastparse(capture.get(3).unwrap().as_bytes());
            let r = fastparse(capture.get(4).unwrap().as_bytes());
            sum += l * r;
        }
    }
    sum
}

fn fastparse(slice: &[u8]) -> i64 {
    if slice.len() == 3 {
        slice[0] as i64 * 100 + slice[1] as i64 * 10 + slice[2] as i64 - b'0' as i64 * 111
    } else if slice.len() == 2 {
        slice[0] as i64 * 10 + slice[1] as i64 - b'0' as i64 * 11
    } else if slice.len() == 1 {
        slice[0] as i64 - b'0' as i64
    } else {
        str::from_utf8(slice).unwrap().parse::<i64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
