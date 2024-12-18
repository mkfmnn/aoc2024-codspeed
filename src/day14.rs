const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn parse_line(input: &str) -> ((isize, isize), (isize, isize)) {
    fn parse_pair(input: &str) -> (isize, isize) {
        let start = input.find('=').unwrap() + 1;
        let (a, b) = input[start..].split_once(',').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }
    let (a, b) = input.split_once(' ').unwrap();
    (parse_pair(a), parse_pair(b))
}

pub fn part1(input: &str) -> usize {
    let mut quadrants = [0usize; 4];
    for line in input.lines() {
        let (mut pos, vel) = parse_line(line);
        pos.0 = (pos.0 + vel.0 * 100).rem_euclid(WIDTH);
        pos.1 = (pos.1 + vel.1 * 100).rem_euclid(HEIGHT);
        if pos.0 == WIDTH / 2 || pos.1 == HEIGHT / 2 {
            continue;
        }
        let quad =
            (if pos.0 < WIDTH / 2 { 0 } else { 1 }) + (if pos.1 < HEIGHT / 2 { 0 } else { 2 });
        quadrants[quad] += 1;
    }
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(_bytes: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input14.txt");

    #[test]
    fn test_part1() {
        assert_eq!(218619120, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(INPUT));
    }
}
