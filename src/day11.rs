use std::collections::HashMap;

pub fn parse(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
}

pub fn part1(input: &str) -> usize {
    let mut cache = HashMap::<(usize, usize), usize>::with_capacity(1_000);
    parse(input)
        .map(|n| expand::<100_000>(n, 25, &mut cache))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cache = HashMap::<(usize, usize), usize>::with_capacity(100_000);
    parse(input)
        .map(|n| expand::<100_000>(n, 75, &mut cache))
        .sum()
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
