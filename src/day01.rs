use crate::util::read_input;
use crate::read_lines_u32;
use std::collections::HashMap;
use std::iter::zip;

pub(crate) const FILE_PATH: &str = "input/day01.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day01_sample.txt";


pub(crate) fn part1(file_path: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for mut iter in read_lines_u32!(file_path) {
        insert_sorted(&mut left, iter.next().unwrap());
        insert_sorted(&mut right, iter.next().unwrap());
    }
    zip(left, right).map(|(a, b)| a.abs_diff(b)).sum()
}


pub(crate) fn part2(file_path: &str) -> u32 {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    for mut iter in read_lines_u32!(file_path) {
        insert_plus_one(&mut left, iter.next().unwrap());
        insert_plus_one(&mut right, iter.next().unwrap());
    }
    left.iter()
        .map(|(l, &lcount)| l * lcount * right.get(l).unwrap_or(&0))
        .sum()
}

fn insert_plus_one(map: &mut HashMap<u32, u32>, key: u32) {
    map.entry(key).and_modify(|x| *x += 1).or_insert(1);
}

fn insert_sorted<T: PartialOrd>(vec: &mut Vec<T>, element: T) {
    if vec.is_empty() || vec[vec.len() - 1] <= element {
        return vec.push(element);
    }
    for i in 0..vec.len() {
        if element < vec[i] {
            vec.insert(i, element);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 11);
        assert_eq!(part1(FILE_PATH), 1603498);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 31);
        assert_eq!(part2(FILE_PATH), 25574739);
    }
}