use crate::read_lines;
use crate::util::read_input;
use itertools::Itertools;
use std::iter::{repeat_n, Iterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::str::ParallelString;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day07.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day07_sample.txt";

#[derive(Debug, Clone)]
enum Op {
    Add, Mul,
    Concat,
}


pub(crate) fn part1(file_path: &str) -> u64 {
    let mut result = 0;
    for line in read_lines!(file_path) {
        let [sum, operands] = line.split(": ").collect::<Vec<_>>()[..] else { unreachable!() };
        let sum: u64 = sum.parse().unwrap();
        let operands: Vec<_> = operands.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

        let possible_operations = repeat_n([Op::Add, Op::Mul], operands.len() - 1).multi_cartesian_product().collect_vec();

        result += solve(sum, operands, possible_operations);
    }
    result
}

pub(crate) fn part2(file_path: &str) -> u64 {
    let content = read_input(file_path);
    content.par_lines().map(|line| {
        let [sum, operands] = line.split(": ").collect::<Vec<_>>()[..] else { unreachable!() };
        let sum: u64 = sum.parse().unwrap();
        let operands: Vec<_> = operands.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

        let possible_operations = repeat_n([Op::Add, Op::Mul, Op::Concat], operands.len() - 1).multi_cartesian_product().collect_vec();

        solve(sum, operands, possible_operations)
    }).sum()
}

fn solve(sum: u64, operands: Vec<u64>, possible_operations: Vec<Vec<Op>>) -> u64 {
    possible_operations.par_iter().find_any(|operations| {
        assert_eq!(operands.len() - 1, operations.len(), "{:?} {:?}", operands, operations);
         sum == operands.iter().copied().enumerate().reduce(|(i, acc), (j, operand)| match operations[i] {
            Op::Add => (j, acc + operand),
            Op::Mul => (j, acc * operand),
            Op::Concat => (j, concat_u64(acc, operand)),
        }).unwrap().1 
    }).map(|_| sum).unwrap_or(0)
}

fn concat_u64(a: u64, b: u64) -> u64 {
    (a.to_string() + &*b.to_string()).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 3749);
        assert_eq!(part1(FILE_PATH), 12553187650171);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 11387);
        assert_eq!(part2(FILE_PATH), 96779702119491);
    }
}
