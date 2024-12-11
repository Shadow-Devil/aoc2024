use crate::util::read_input;
use std::collections::HashMap;
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day11.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day11_sample.txt";

pub(crate) fn part1(file_path: &str) -> usize {
    let content = read_input(file_path);
    let mut stones: Vec<u64> = content.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let blinks = 25;

    for i in 1..=blinks {
        stones = stones.iter().flat_map(|stone| {
            if *stone == 0 {
                Vec::from([1])
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let stone = stone.to_string();
                let (l, r) = stone.split_at(stone.len() / 2);
                assert!(!l.is_empty(), "{}", stone);
                assert!(!r.is_empty(), "{}", stone);
                Vec::from([l.parse().expect(l), r.trim_start_matches('0').parse().unwrap_or(0)])
            } else {
                Vec::from([*stone * 2024])
            }
        }).collect();
    }
    stones.len()
}


pub(crate) fn part2(file_path: &str) -> u64 {
    let content = read_input(file_path);
    let mut stones_vec: Vec<u64> = content.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in stones_vec {
        *stones.entry(stone).or_default() += 1;
    }

    let blinks = 75;

    for i in 1..=blinks {
        let mut stones_new = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                *stones_new.entry(1).or_default() += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let stone = stone.to_string();
                let (l, r) = stone.split_at(stone.len() / 2);
                assert!(!l.is_empty(), "{}", stone);
                assert!(!r.is_empty(), "{}", stone);

                *stones_new.entry(l.parse().expect(l)).or_default() += count;
                *stones_new.entry(r.trim_start_matches('0').parse().unwrap_or(0)).or_default() += count;
            } else {
                *stones_new.entry(stone * 2024).or_default() += count;
            }
        }

        stones = stones_new;
    }
    stones.values().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 55312);
        assert_eq!(part1(FILE_PATH), 231278);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 55312);
        assert_eq!(part2(FILE_PATH), 274229228071551);
    }
}
