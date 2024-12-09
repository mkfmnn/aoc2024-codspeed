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

struct Block {
    id: usize,
    start: usize,
    len: usize,
}

struct FreeBlock {
    start: usize,
    len: usize,
}

fn part2_inner(mut bytes: &[u8]) -> usize {
    if bytes[bytes.len() - 1] == b'\n' {
        bytes = &bytes[..bytes.len() - 1];
    }
    let mut blocks = Vec::<Block>::with_capacity(10001);
    let mut free_blocks = Vec::<FreeBlock>::with_capacity(10000);
    let mut i = 0;
    let mut offset = 0;

    let len = (bytes[i] - b'0') as usize;
    blocks.push(Block {
        id: i / 2,
        start: offset,
        len,
    });
    offset += len;
    i += 1;
    while i < bytes.len() {
        let len = (bytes[i] - b'0') as usize;
        free_blocks.push(FreeBlock { start: offset, len });
        offset += len;
        i += 1;
        let len = (bytes[i] - b'0') as usize;
        blocks.push(Block {
            id: i / 2,
            start: offset,
            len,
        });
        offset += len;
        i += 1;
    }
    for block in blocks.iter_mut().rev() {
        for free in free_blocks.iter_mut() {
            if free.start > block.start {
                break;
            }
            if free.len >= block.len {
                block.start = free.start;
                free.start += block.len;
                free.len -= block.len;
                break;
            }
        }
    }
    let mut checksum = 0;
    for block in blocks {
        checksum += block.id * block.len * (2 * block.start + block.len - 1);
    }
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
    fn test_part2() {
        assert_eq!(6361380647183, part2(INPUT));
    }
}
