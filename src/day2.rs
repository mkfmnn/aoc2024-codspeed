use memchr::memchr;

pub fn parse_lines(input: &str) -> usize {
    let mut safe = 0;
    for _ in input_iter(input) {
        safe += 1;
    }
    safe
}

pub fn parse_all(input: &str) -> usize {
    let mut total = 0;
    for levels in input_iter(input) {
        for i in levels {
            total += i;
        }
    }
    total as usize
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
    let mut deltas = Vec::<i32>::with_capacity(10);
    for mut levels_it in input_iter(input) {
        deltas.clear();
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
    let first = levels.next().unwrap();
    let Some(mut prev) = levels.next() else {
        return true;
    };
    let d = prev - first;
    let signum = d.signum();
    let abs = d.abs();
    if signum == 0 || abs < 1 || abs > 3 {
        return false;
    }
    while let Some(next) = levels.next() {
        let d = next - prev;
        let abs = d.abs();
        if d.signum() != signum || abs < 1 || abs > 3 {
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

struct IterLines<'a> {
    input: &'a [u8],
}

impl<'a> IterLines<'a> {
    fn new(input: &[u8]) -> IterLines {
        IterLines { input }
    }
}

impl<'a> Iterator for IterLines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        match memchr(b'\n', self.input) {
            Some(idx) => {
                let ret = &self.input[..idx];
                self.input = &self.input[idx + 1..];
                Some(ret)
            }
            None => {
                let ret = self.input;
                self.input = &[];
                Some(ret)
            }
        }
    }
}

struct IterInts<'a> {
    line: &'a [u8],
}

impl<'a> IterInts<'a> {
    fn parse(line: &[u8]) -> IterInts {
        IterInts { line: line }
    }
}

impl<'a> Iterator for IterInts<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.len() > 2 {
            if self.line[2] == b' ' {
                let result = self.line[0] as i32 * 10 + self.line[1] as i32 - b'0' as i32 * 11;
                self.line = &self.line[3..];
                return Some(result);
            } else if self.line[1] == b' ' {
                let result = self.line[0] as i32 - b'0' as i32;
                self.line = &self.line[2..];
                return Some(result);
            }
        }
        if self.line.len() == 2 {
            let result = self.line[0] as i32 * 10 + self.line[1] as i32 - b'0' as i32 * 11;
            self.line = &[];
            return Some(result);
        } else if self.line.len() == 1 {
            let result = self.line[0] as i32 - b'0' as i32;
            self.line = &[];
            return Some(result);
        }
        if self.line.len() == 0 {
            None
        } else {
            // Fallback parser--shouldn't normally get used
            let mut n = 0;
            let mut i = 0;
            while i < self.line.len() {
                if self.line[i] == b' ' {
                    i += 1;
                    break;
                }
                n *= 10;
                n += (self.line[i] - b'0') as i32;
                i += 1;
            }
            self.line = &self.line[i..];
            Some(n)
        }
    }
}

fn input_iter(input: &str) -> impl Iterator<Item = impl Iterator<Item = i32> + '_> {
    IterLines::new(input.as_bytes()).map(IterInts::parse)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("data/input2.txt").unwrap();
        assert_eq!(324651, parse_all(&input));
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
