use std::{cmp::Reverse, collections::BinaryHeap};

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    let mut i = 0usize;
    let mut j = bytes.len() - 1;
    if bytes[j] == b'\n' {
        j -= 1;
    }
    let mut offset = 0;
    let mut checksum = 0;
    let mut back_id = 0;
    let mut back_remaining = 0;
    // alternate reading from the front and from the back until one pointer would cross the other
    while i <= j {
        let front_id = i / 2;
        let len = (bytes[i] - b'0') as usize;
        checksum += front_id * len * (2 * offset + len - 1);
        offset += len;
        i += 1;
        let mut free_len = (bytes[i] - b'0') as usize;
        while free_len > 0 || j <= i {
            let used = if back_remaining == 0 {
                if j <= i {
                    break;
                }
                back_id = j / 2;
                back_remaining = (bytes[j] - b'0') as usize;
                debug_assert!(back_remaining > 0);
                j -= 2;
                usize::min(free_len, back_remaining)
            } else if j <= i {
                back_remaining
            } else {
                usize::min(free_len, back_remaining)
            };
            checksum += back_id * used * (2 * offset + used - 1);
            free_len = free_len.saturating_sub(used);
            back_remaining -= used;
            offset += used;
        }
        i += 1;
    }
    checksum / 2
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(mut bytes: &[u8]) -> usize {
    if bytes[bytes.len() - 1] == b'\n' {
        bytes = &bytes[..bytes.len() - 1];
    }
    let mut free_blocks: [BinaryHeap<Reverse<usize>>; 10] =
        core::array::from_fn(|_| BinaryHeap::with_capacity(1000));
    let mut free_idx = 0;
    let mut offset = 0;
    let mut end_idx = bytes.len() - 1;
    let mut end_offset = usize::MAX; // initially unknown; points to *end* of current index
    if bytes[end_idx] == b'\n' {
        end_idx -= 1;
    }
    let mut checksum = 0;
    'outer: loop {
        let id = end_idx / 2;
        let len = (bytes[end_idx] - b'0') as usize;
        if end_idx == free_idx {
            end_offset = offset + len as usize;
        }
        // find the first free block that this block can fit in
        // first check the free lists
        let mut best = usize::MAX;
        let mut best_len = 0;
        for i in len..=9 {
            if let Some(s) = free_blocks[i].peek() {
                if s.0 < best {
                    best_len = i;
                    best = s.0;
                }
            }
        }
        if best != usize::MAX && best < end_offset {
            let popped = free_blocks[best_len].pop().unwrap();
            debug_assert_eq!(popped.0, best);
            let start = best;
            checksum += id * len * (2 * start + len - 1);
            if best_len > len {
                free_blocks[best_len - len].push(Reverse(best + len));
            }
            end_offset -= len;
            end_offset -= (bytes[end_idx - 1] - b'0') as usize;
            end_idx -= 2;
            if end_idx == 0 {
                break;
            }
            continue;
        }
        // if no block in free lists is big enough, scan for more
        while end_idx >= free_idx {
            offset += (bytes[free_idx] - b'0') as usize;
            free_idx += 1;
            let mut free_start = offset;
            let mut free_len = (bytes[free_idx] - b'0') as usize;
            offset += free_len;
            free_idx += 1;
            if end_idx == free_idx {
                end_offset = offset + len as usize;
            }

            // is it big enough? if so use it immediately
            if free_len >= len {
                checksum += id * len * (2 * free_start + len - 1);
                free_len -= len;
                free_start += len;
                if free_len > 0 {
                    free_blocks[free_len].push(Reverse(free_start));
                }
                end_idx -= 2;
                end_offset -= len + (bytes[end_idx + 1] - b'0') as usize;
                continue 'outer;
            } else if free_len > 0 {
                free_blocks[free_len].push(Reverse(free_start));
            }
        }
        // if we got here, no free block found, keep it in-place
        checksum += id * len * (2 * end_offset - len - 1);
        end_idx -= 2;
        if end_idx == 0 {
            break;
        }
        end_offset -= len + (bytes[end_idx + 1] - b'0') as usize;
    }
    // don't have to deal with 0!
    checksum / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input9.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(1928, part1("2333133121414131402"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(6337367222422, part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2858, part2("2333133121414131402"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6361380647183, part2(INPUT));
    }
}
