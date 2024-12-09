const DIM: usize = 50;
const LINE_LEN: usize = DIM + 1;
const MAP_LEN: usize = DIM * LINE_LEN;

const LUT: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 255;
    while i != 0 {
        table[i as usize] = match i {
            b'0'..=b'9' => i - b'0' + 1,
            b'A'..=b'Z' => i - b'A' + 1 + 10,
            b'a'..=b'z' => i - b'a' + 1 + 10 + 26,
            _ => 255,
        };
        i -= 1;
    }
    table[0] = 255;
    table
};

pub fn part1(input: &str) -> usize {
    inner::<false>(input.as_bytes())
}

pub fn part2(input: &str) -> usize {
    inner::<true>(input.as_bytes())
}

fn byte_offset_to_coord(i: usize) -> (i8, i8) {
    ((i / LINE_LEN) as i8, (i % LINE_LEN) as i8)
}

fn inner<const REPEAT: bool>(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), MAP_LEN);
    let mut stations = [[(0i8, 0i8); 4]; 64];
    let mut station_count = [0usize; 64];
    for (i, &c) in bytes.iter().enumerate() {
        let s = LUT[c as usize] as usize;
        if s != 255 {
            stations[s][station_count[s]] = byte_offset_to_coord(i);
            station_count[s] += 1;
        }
    }

    let mut antinodes = [0u64; DIM];
    for s in 1..=62 {
        for i in 0..station_count[s] {
            for j in 0..station_count[s] {
                if i == j {
                    continue;
                }
                let a = stations[s][i];
                let b = stations[s][j];
                if REPEAT {
                    let delta = (b.0 - a.0, b.1 - a.1);
                    let mut antinode = b;
                    while antinode.0 >= 0
                        && antinode.0 < DIM as i8
                        && antinode.1 >= 0
                        && antinode.1 < DIM as i8
                    {
                        antinodes[antinode.0 as usize] |= 1 << antinode.1;
                        antinode = (antinode.0 + delta.0, antinode.1 + delta.1);
                    }
                } else {
                    let antinode = (b.0 + b.0 - a.0, b.1 + b.1 - a.1);
                    if antinode.0 >= 0
                        && antinode.0 < DIM as i8
                        && antinode.1 >= 0
                        && antinode.1 < DIM as i8
                    {
                        antinodes[antinode.0 as usize] |= 1 << antinode.1;
                    }
                }
            }
        }
    }
    antinodes.into_iter().map(|a| a.count_ones()).sum::<u32>() as usize
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
