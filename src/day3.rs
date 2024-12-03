use regex::Regex;

pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let [l, r] = c.extract().1;
            l.parse::<i64>().unwrap() * r.parse::<i64>().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for capture in re.captures_iter(input) {
        if capture.get(1).is_some() {
            enabled = true;
        } else if capture.get(2).is_some() {
            enabled = false;
        } else if enabled {
            let l = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let r = capture.get(4).unwrap().as_str().parse::<i64>().unwrap();
            sum += l * r;
        }
    }
    sum
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
