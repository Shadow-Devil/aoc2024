use crate::read_lines;
use crate::util::read_input;
use itertools::Itertools;
use std::iter::{repeat_n, Iterator};

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
    let mut result = 0;
    for line in read_lines!(file_path) {
        let [sum, operands] = line.split(": ").collect::<Vec<_>>()[..] else { unreachable!() };
        let sum: u64 = sum.parse().unwrap();
        let operands: Vec<_> = operands.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

        let possible_operations = repeat_n([Op::Add, Op::Mul, Op::Concat], operands.len() - 1).multi_cartesian_product().collect_vec();

        result += solve(sum, operands, possible_operations);
    }
    result
}

fn solve(sum: u64, operands: Vec<u64>, possible_operations: Vec<Vec<Op>>) -> u64 {
    for operations in possible_operations {
        assert_eq!(operands.len() - 1, operations.len(), "{:?} {:?}", operands, operations);
        if sum == operands.iter().copied().enumerate().reduce(|(i, acc), (j, operand)| match operations[i] {
            Op::Add => (j, acc + operand),
            Op::Mul => (j, acc * operand),
            Op::Concat => (j, concat_u64(acc, operand)),
        }).unwrap().1 {
            return sum;
        }
    }
    0
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
        assert_eq!(part1(FILE_PATH), 5409);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 6);
        assert_eq!(part2(FILE_PATH), 2022);
    }
}
