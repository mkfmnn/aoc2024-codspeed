pub fn parse(input: &str) -> usize {
    let mut safe = 0;
    for _ in input_iter(input) {
        safe += 1;
    }
    safe
}

pub fn part1(input: &str) -> usize {
    let mut safe = 0;
    for levels in input_iter(input) {
        if is_safe(levels) {
            safe += 1;
        }
    }
    safe
}

pub fn part2(input: &str) -> i32 {
    let mut safe = 0;
    for levels_it in input_iter(input) {
        let levels = levels_it.collect::<Vec<_>>();
        if is_safe(levels.iter().copied()) {
            safe += 1;
        } else {
            for i in 0..levels.len() {
                let mut test_levels = levels.clone();
                test_levels.remove(i);
                if is_safe(test_levels.into_iter()) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

fn is_safe(mut levels: impl Iterator<Item = i32>) -> bool {
    let mut asc_ok = true;
    let mut desc_ok = true;
    let mut prev = levels.next().unwrap();
    while let Some(next) = levels.next() {
        let d = next - prev;
        if d > 0 && d <= 3 {
            if !asc_ok {
                return false;
            }
            desc_ok = false;
        } else if d < 0 && d >= -3 {
            if !desc_ok {
                return false;
            }
            asc_ok = false;
        } else {
            return false;
        }
        prev = next;
    }
    return true;
}

fn fastparse(input: &[u8]) -> i32 {
    if input.len() == 2 {
        input[0] as i32 * 10 + input[1] as i32 - b'0' as i32 * 11
    } else if input.len() == 1 {
        input[0] as i32 - b'0' as i32
    } else {
        let mut n = 0;
        for c in input {
            n *= 10;
            n += (c - b'0') as i32;
        }
        n
    }
}

fn input_iter(input: &str) -> impl Iterator<Item = impl Iterator<Item = i32> + '_> + '_ {
    let bytes = input.trim_end_matches('\n').as_bytes();
    bytes
        .split(|&b| b == b'\n')
        .map(|l| l.split(|&b| b == b' ').map(fastparse))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("data/input2.txt").unwrap();
        assert_eq!(1000, parse(&input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input2.txt").unwrap();
        assert_eq!(279, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input2.txt").unwrap();
        assert_eq!(343, part2(&input));
    }
}
