use std::collections::{HashMap, HashSet};

const DIM: usize = 50;
const LINE_LEN: usize = DIM + 1;

pub fn part1(input: &str) -> usize {
    inner(input.as_bytes(), false)
}

pub fn part2(input: &str) -> usize {
    inner(input.as_bytes(), true)
}

fn inner(bytes: &[u8], repeat: bool) -> usize {
    let mut stations = HashMap::<u8, Vec<(usize, usize)>>::new();
    for y in 0..DIM {
        for x in 0..DIM {
            let c = bytes[x + LINE_LEN * y];
            if c != b'.' {
                stations.entry(c).or_default().push((x, y));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for antennae in stations.values() {
        for &(a_x, a_y) in antennae {
            for &(b_x, b_y) in antennae {
                let (d_x, d_y) = ((b_x as isize - a_x as isize), (b_y as isize - a_y as isize));
                if d_x == 0 && d_y == 0 {
                    continue;
                }
                let mut antinode = (b_x as isize + d_x, b_y as isize + d_y);
                while antinode.0 >= 0
                    && antinode.0 < DIM as isize
                    && antinode.1 >= 0
                    && antinode.1 < DIM as isize
                {
                    antinodes.insert(antinode);
                    if !repeat {
                        break;
                    }
                    antinode = (antinode.0 + d_x, antinode.1 + d_y);
                }
                if repeat {
                    antinodes.insert((b_x as isize, b_y as isize));
                }
            }
        }
    }
    antinodes.into_iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input8.txt");

    #[test]
    fn test_part1() {
        assert_eq!(379, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1339, part2(INPUT));
    }
}
