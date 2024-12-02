use crate::read_lines;
use crate::util::{next_u32, read_input};

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day02.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day02_sample.txt";

const MAX_DIFF: u32 = 3;
const MIN_DIFF: u32 = 1;

pub(crate) fn part1(file_path: &str) -> u32 {
    let mut result = 0;
    for line in read_lines!(file_path) {
        result += solve(line.split_ascii_whitespace().map(|x| x.parse().unwrap()));
    }
    result
}

fn solve<I: Iterator<Item = u32>>(mut nums: I) -> u32 {
    let x = nums.next().unwrap();
    let mut y = nums.next().unwrap();

    let asc = x < y;
    if x.abs_diff(y) > MAX_DIFF || x.abs_diff(y) < MIN_DIFF { return 0; }
    
    for num in nums {
        if asc && y < num && y.abs_diff(num) <= MAX_DIFF {
            y = num;
        } else if !asc && num < y && y.abs_diff(num) <= MAX_DIFF {
            y = num;
        } else {
            return 0;
        }
    }
    1
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let mut result = 0;
    for line in read_lines!(file_path) {
        let nums: Vec<u32> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        if solve(nums.iter().copied()) == 1 { 
            result += 1;
            continue
        }
        for i in 0..nums.len() {
            let mut nums = nums.to_owned();
            nums.remove(i);
            if solve(nums.iter().copied()) == 1 {
                result += 1;
                break
            }
        }
    }
    result
}
