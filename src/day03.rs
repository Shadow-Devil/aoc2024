use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::fs;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day03.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day03_sample.txt";

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
}

pub(crate) fn part1(file_path: &str) -> u32 {
    let content = fs::read_to_string(file_path).unwrap().replace("\n", "");
    solve(&content)
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let mut result: u32 = 0;
    let content = fs::read_to_string(file_path).unwrap().replace("\n", "");
    let mut rest = &*content;
    result += solve_part2(&mut rest);
    while !rest.is_empty() {
        rest = rest.split_once("do()").map_or("", |x| x.1);
        result += solve_part2(&mut rest);
    }
    result
}

fn solve(line: &str) -> u32 {
    PATTERN
        .captures_iter(line)
        .filter_map(|x| {
            parse_u32(x.get(1)).and_then(|z: u32| parse_u32(x.get(2)).map(|z2: u32| z2 * z))
        })
        .sum::<u32>()
}

fn parse_u32(x: Option<Match>) -> Option<u32> {
    x.and_then(|y| y.as_str().parse().ok())
}

fn solve_part2(rest: &mut &str) -> u32 {
    let (a, b) = rest.split_once("don't()").unwrap_or((rest, ""));
    *rest = b;
    solve(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 161);
        assert_eq!(part1(FILE_PATH), 173419328);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 48);
        assert_eq!(part2(FILE_PATH), 90669332);
    }
}
