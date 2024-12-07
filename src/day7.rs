pub fn part1(input: &str) -> u64 {
    part_inner(input, recurse1)
}

pub fn part2(input: &str) -> u64 {
    part_inner(input, recurse2)
}

pub fn part_inner<F>(input: &str, f: F) -> u64
where
    F: Fn(u64, u64, &[u64]) -> bool,
{
    let mut sum = 0;
    for line in input.lines() {
        let (target, rest) = line.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums = rest
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let (&first, rest) = nums.split_first().unwrap();
        if f(target, first, rest) {
            sum += target;
        }
    }
    sum
}

fn recurse1(target: u64, accum: u64, rest: &[u64]) -> bool {
    let Some((&first, rest)) = rest.split_first() else {
        return target == accum;
    };
    recurse1(target, accum + first, rest) || recurse1(target, accum * first, rest)
}

fn recurse2(target: u64, accum: u64, rest: &[u64]) -> bool {
    let Some((&first, rest)) = rest.split_first() else {
        return target == accum;
    };
    recurse2(target, accum + first, rest)
        || recurse2(target, accum * first, rest)
        || recurse2(target, concat(accum, first), rest)
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse::<u64>().unwrap()
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
