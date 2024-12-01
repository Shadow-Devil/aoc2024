use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub(crate) fn part1() -> u32 {
    let file_path = "input/day01.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut iter = line.split_ascii_whitespace();
        let l = iter.next();
        if l.is_none() {
            continue;
        }
        insert_sorted(&mut left, l.unwrap().parse().unwrap());
        assert!(left.is_sorted());

        let r: i32 = iter.next().unwrap().parse().unwrap();
        insert_sorted(&mut right, r);
        assert!(right.is_sorted());
        assert_eq!(left.len(), right.len());
    }
    assert_eq!(left.len(), right.len());
    let result: u32 = zip(left, right).map(|(a, b)| a.abs_diff(b)).sum();

    result
}

pub(crate) fn part2() -> u32 {
    let file_path = "input/day01.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();


    for line in contents.lines() {
        let mut iter = line.split_ascii_whitespace();
        let l = iter.next();
        if l.is_none() {
            continue;
        }
        let l: u32 = l.unwrap().parse().unwrap();
        left.entry(l).and_modify(|x| *x += 1).or_insert(1);

        let r: u32 = iter.next().unwrap().parse().unwrap();
        right.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }
    let result: u32 = left.iter()
        .map(|(l, lcount)| l * lcount * right.get(l).unwrap_or(&0))
        .sum();
    result
}


fn insert_sorted(vec: &mut Vec<i32>, element: i32) {
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
