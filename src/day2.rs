pub fn part1(input: &str) -> usize {
    let mut safe = 0;
    for levels in input_iter(input) {
        if is_safe(&levels) {
            safe += 1;
        }
    }
    safe
}

pub fn part2(input: &str) -> i32 {
    let mut safe = 0;
    for levels in input_iter(input) {
        if is_safe(&levels) {
            safe += 1;
        } else {
            for i in 0..levels.len() {
                let mut test_levels = levels.clone();
                test_levels.remove(i);
                if is_safe(&test_levels) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

fn is_safe(levels: &[i32]) -> bool {
    let mut it = levels.iter().copied();
    let mut asc_ok = true;
    let mut desc_ok = true;
    let mut prev = it.next().unwrap();
    while let Some(next) = it.next() {
        let d = next - prev;
        if d > 0 {
            if !asc_ok {
                return false;
            }
            desc_ok = false;
            if d > 3 {
                return false;
            }
        } else if d < 0 {
            if !desc_ok {
                return false;
            }
            asc_ok = false;
            if d < -3 {
                return false;
            }
        } else {
            return false;
        }
        prev = next;
    }
    return true;
}

fn input_iter(input: &str) -> InputIter {
    InputIter {
        input: input.as_bytes(),
        next: 0,
    }
}

struct InputIter<'a> {
    input: &'a [u8],
    next: usize,
}

impl Iterator for InputIter<'_> {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.next;
        if i >= self.input.len() {
            return None;
        }
        let mut v = Vec::with_capacity(10);
        let mut n = 0;
        loop {
            if self.input[i] == b' ' {
                v.push(n);
                n = 0;
            } else if self.input[i] == b'\n' {
                v.push(n);
                break;
            } else {
                n *= 10;
                n += (self.input[i] - b'0') as i32;
            }
            i += 1;
        }
        i += 1;
        self.next = i;
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
