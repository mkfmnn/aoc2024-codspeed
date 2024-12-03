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
    for mut levels_it in input_iter(input) {
        let mut deltas = Vec::with_capacity(10);
        let mut prev = levels_it.next().unwrap();
        while let Some(curr) = levels_it.next() {
            deltas.push(curr - prev);
            prev = curr;
        }
        if is_safe_dampened::<1, 3>(&deltas) || is_safe_dampened::<-3, -1>(&deltas) {
            safe += 1;
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

fn is_safe_dampened<const MIN: i32, const MAX: i32>(deltas: &[i32]) -> bool {
    let mut dampened = false;
    let mut i = 0;
    while i < deltas.len() {
        let this = deltas[i];
        if this < MIN || this > MAX {
            if dampened {
                return false;
            }
            if i + 1 == deltas.len() {
                return true;
            }
            let next = deltas[i + 1];
            let merged = this + next;
            if merged >= MIN && merged <= MAX {
                dampened = true;
                i += 1;
            } else if next < MIN || next > MAX {
                return false;
            } else {
                if i == 0 {
                    dampened = true;
                } else {
                    let prev = deltas[i - 1];
                    let merged = this + prev;
                    if merged >= MIN && merged <= MAX {
                        dampened = true;
                    } else {
                        return false;
                    }
                }
            }
        }
        i += 1;
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

fn input_iter(input: &str) -> impl Iterator<Item = impl Iterator<Item = i32> + '_> {
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
