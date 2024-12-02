pub fn part1(input: &str) -> usize {
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<_> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        if is_safe(&levels) {
            safe += 1;
        }
    }
    safe
}

pub fn part2(input: &str) -> i32 {
    let mut safe = 0;
    for line in input.lines() {
        let levels: Vec<_> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
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
    let deltas: Vec<_> = levels.windows(2).map(|w| w[1] - w[0]).collect();
    deltas.iter().all(|&d| d > 0 && d <= 3) || deltas.iter().all(|&d| d < 0 && d >= -3)
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
