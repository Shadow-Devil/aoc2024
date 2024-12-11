use crate::read_lines_u32;
use crate::util::read_input;
use crate::util::Countable;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day02.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day02_sample.txt";

const MAX_DIFF: u32 = 3;

pub(crate) fn part1(file_path: &str) -> u32 {
    read_lines_u32!(file_path).count_all(solve) as u32
}

pub(crate) fn part2(file_path: &str) -> u32 {
    read_lines_u32!(file_path).count_all(|iter| {
        let nums: Vec<u32> = iter.collect();
        if solve(nums.iter().copied()) {
            return true;
        }
        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);
            if solve(nums.iter().copied()) {
                return true;
            }
        }
        false
    }) as u32
}

fn solve(mut nums: impl Iterator<Item = u32>) -> bool {
    let x = nums.next().unwrap();
    let mut y = nums.next().unwrap();

    let asc = x < y;
    if !valid_pair(asc, x, y) {
        return false;
    }

    for num in nums {
        if !valid_pair(asc, y, num) {
            return false;
        }
        y = num;
    }
    true
}

fn valid_pair(asc: bool, x: u32, y: u32) -> bool {
    x.abs_diff(y) <= MAX_DIFF && (asc && x < y || !asc && y < x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 2);
        assert_eq!(part1(FILE_PATH), 534);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 4);
        assert_eq!(part2(FILE_PATH), 577);
    }
}
