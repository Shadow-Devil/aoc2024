use crate::util::read_input;
use std::cmp::min;
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day09.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day09_sample.txt";

enum Block {
    File(u32, u64),
    Free(u32),
}

pub(crate) fn part1(file_path: &str) -> u64 {
    let content = read_input(file_path);
    let mut disk_map: Vec<Block> = Vec::with_capacity(content.len());
    let mut idx = 0;
    let mut is_file = true;

    for block in content.chars().map(|c| c.to_digit(10).unwrap()) {
        disk_map.push(if is_file {
            // postfix increment is not supported in rust
            idx += 1;
            Block::File(block, idx - 1)
        } else {
            Block::Free(block)
        });
        is_file = !is_file;
    }
    let mut result: u64 = 0;
    let mut pos: u64 = 0;
    while !disk_map.is_empty() {
        match disk_map[0] {
            Block::File(len, idx) => {
                for _x in 0..len {
                    result += idx * pos;
                    pos += 1;
                }
            }
            Block::Free(mut len) => {
                while len > 0 && !disk_map.is_empty() {
                    match disk_map[disk_map.len() - 1] {
                        Block::File(mut len1, idx) => {
                            for _x in 0..min(len, len1) {
                                result += idx * pos;
                                pos += 1;
                                len -= 1;
                                len1 -= 1;
                            }
                            disk_map.remove(disk_map.len() - 1);
                            if len1 > 0 {
                                disk_map.push(Block::File(len1, idx))
                            }
                        }
                        Block::Free(_) => {
                            disk_map.remove(disk_map.len() - 1);
                        }
                    }
                }
            }
        }
        disk_map.remove(0);
    }

    result
}

pub(crate) fn part2(file_path: &str) -> u64 {
    let content = read_input(file_path);
    let mut disk_map: Vec<Block> = Vec::with_capacity(content.len());
    let mut idx = 0;
    let mut is_file = true;

    for block in content.chars().map(|c| c.to_digit(10).unwrap()) {
        disk_map.push(if is_file {
            // postfix increment is not supported in rust
            idx += 1;
            Block::File(block, idx - 1)
        } else {
            Block::Free(block)
        });
        is_file = !is_file;
    }
    let mut result: u64 = 0;
    let mut pos: u64 = 0;
    while !disk_map.is_empty() {
        match disk_map[0] {
            Block::File(len, idx) => {
                println!("Adding {} {} {}", len, idx, pos);
                for _x in 0..len {
                    result += idx * pos;
                    pos += 1;
                }
            }
            Block::Free(mut len) => {
                let mut i_end = disk_map.len() - 1;
                while len > 0
                    && disk_map.iter().any(|b| match b {
                        Block::File(_, _) => true,
                        Block::Free(_) => false,
                    })
                    && i_end > 0
                {
                    match disk_map[i_end] {
                        Block::File(len1, idx) => {
                            if len >= len1 {
                                println!("Adding {} {} {}", len1, idx, pos);
                                for _x in 0..len1 {
                                    result += idx * pos;
                                    pos += 1;
                                    len -= 1;
                                }
                                disk_map.remove(i_end);
                                disk_map.insert(i_end, Block::Free(len1));
                            }
                        }
                        Block::Free(_) => {
                            //disk_map.remove(i_end);
                        }
                    }
                    i_end -= 1;
                }
                if len > 0 {
                    pos += len as u64;
                }
            }
        }
        disk_map.remove(0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 1928);
        assert_eq!(part1(FILE_PATH), 6211348208140);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 2858);
        assert_eq!(part2(FILE_PATH), 6239783302560);
    }
}
