use std::simd::{
    cmp::{SimdPartialEq, SimdPartialOrd},
    i8x64,
    num::SimdUint,
};

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

#[inline(always)]
fn set_station(
    stations: &mut [i8; 512],
    station_count: &mut [usize; 64],
    station: usize,
    bytes_offset: usize,
) {
    debug_assert!((1..=62).contains(&station));
    let (x, y) = (
        (bytes_offset % LINE_LEN) as i8,
        (bytes_offset / LINE_LEN) as i8,
    );
    let c = station_count[station];
    stations[(c << 7) + station] = x;
    stations[(c << 7) + 64 + station] = y;
    station_count[station] += 1;
}

type U8xRead = std::simd::u8x32;

fn inner<const REPEAT: bool>(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), MAP_LEN);
    let mut stations_simd: [i8x64; 8] = [i8x64::splat(101); 8];
    let mut station_count = [0usize; 64];
    {
        let stations: &mut [i8; 512] = unsafe { std::mem::transmute(&mut stations_simd) };
        let (prefix, batch, suffix) = bytes.as_simd::<{ U8xRead::LEN }>();
        for (i, &c) in prefix.iter().enumerate() {
            let s = LUT[c as usize] as usize;
            if s != 255 {
                set_station(stations, &mut station_count, s, i);
            }
        }
        let mut offset = prefix.len();
        for c in batch {
            // squash 0-9, A-Z, a-z into [1, 62]
            let d = c
                .simd_gt(U8xRead::splat(b'9'))
                .select(U8xRead::splat(b'A' - 1 - 10), U8xRead::splat(b'0' - 1));
            let d = c
                .simd_gt(U8xRead::splat(b'Z'))
                .select(U8xRead::splat(b'a' - 1 - 36), d);
            let c = c.saturating_sub(d);
            let mut m = c.simd_ne(U8xRead::splat(0)).to_bitmask();
            let a = c.to_array();
            while m != 0 {
                let t = m.trailing_zeros() as usize;
                let s = a[t] as usize;
                let i = offset + t;

                set_station(stations, &mut station_count, s, i);

                m &= !(1 << t);
            }
            offset += U8xRead::LEN;
        }
        for (i, &c) in suffix.iter().enumerate() {
            let s = LUT[c as usize] as usize;
            if s != 255 {
                set_station(stations, &mut station_count, s, i);
                station_count[s] += 1;
            }
        }
    }

    let mut antinodes = [0u64; DIM];
    if !REPEAT {
        let min = i8x64::splat(0);
        let max = i8x64::splat(DIM as i8);
        for i in 0..4 {
            for j in i + 1..4 {
                let ax = stations_simd[i << 1];
                let ay = stations_simd[(i << 1) + 1];
                let bx = stations_simd[j << 1];
                let by = stations_simd[(j << 1) + 1];
                let dx = bx - ax;
                let dy = by - ay;
                let n1x = bx + dx;
                let n1y = by + dy;
                let mut m =
                    (n1x.simd_ge(min) & n1x.simd_lt(max) & n1y.simd_ge(min) & n1y.simd_lt(max))
                        .to_bitmask();
                let nodes = (n1x.to_array(), n1y.to_array());
                while m != 0 {
                    let t = m.trailing_zeros() as usize;
                    antinodes[nodes.0[t] as usize] |= 1 << nodes.1[t];

                    m &= !(1 << t);
                }
                let n2x = ax - dx;
                let n2y = ay - dy;
                let mut m =
                    (n2x.simd_ge(min) & n2x.simd_lt(max) & n2y.simd_ge(min) & n2y.simd_lt(max))
                        .to_bitmask();
                let nodes = (n2x.to_array(), n2y.to_array());
                while m != 0 {
                    let t = m.trailing_zeros() as usize;
                    antinodes[nodes.0[t] as usize] |= 1 << nodes.1[t];

                    m &= !(1 << t);
                }
            }
        }
    } else {
        let stations: &[i8; 512] = unsafe { std::mem::transmute(&stations_simd) };
        for s in 1..=62 {
            for i in 0..station_count[s] {
                for j in 0..station_count[s] {
                    if i == j {
                        continue;
                    }
                    let a = (stations[(i << 7) + s], stations[(i << 7) + 64 + s]);
                    let b = (stations[(j << 7) + s], stations[(j << 7) + 64 + s]);
                    let delta = (b.0 - a.0, b.1 - a.1);
                    let mut antinode = b;
                    loop {
                        antinodes[antinode.0 as usize] |= 1 << antinode.1;
                        antinode = (antinode.0 + delta.0, antinode.1 + delta.1);
                        if !(antinode.0 >= 0
                            && antinode.0 < DIM as i8
                            && antinode.1 >= 0
                            && antinode.1 < DIM as i8)
                        {
                            break;
                        }
                    }
                    antinode = a;
                    loop {
                        antinodes[antinode.0 as usize] |= 1 << antinode.1;
                        antinode = (antinode.0 - delta.0, antinode.1 - delta.1);
                        if !(antinode.0 >= 0
                            && antinode.0 < DIM as i8
                            && antinode.1 >= 0
                            && antinode.1 < DIM as i8)
                        {
                            break;
                        };
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
