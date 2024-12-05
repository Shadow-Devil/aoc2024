use crate::util::read_input;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day05.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day05_sample.txt";

lazy_static! {
    static ref MAPPING: HashMap<char, u32> =
        HashMap::from([('X', 1), ('M', 2), ('A', 3), ('S', 4),]);
}

pub(crate) fn part1(file_path: &str) -> u32 {
    let content = read_input(file_path);
    if let [page_order_rules, updates] = content.split("\n\n").collect::<Vec<&str>>()[..] {
        let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();
        for page_order in page_order_rules.lines() {
            if let [a, b] = page_order
                .splitn(2, "|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                ordering.entry(a).or_default().push(b)
            } else {
                unreachable!()
            }
        }
        let mut result = 0;
        for update in updates.lines() {
            let mut already_seen: Vec<u32> = Vec::new();
            let prints: Vec<u32> = update.split(",").map(|x| x.parse().unwrap()).collect();
            let mut b = false;
            for print in prints.iter() {
                if !ordering.get(print).map_or(true, |y| {
                    y.iter()
                        .all(|should_be_after| !already_seen.contains(should_be_after))
                }) {
                    b = true;
                    break;
                }
                already_seen.push(*print);
            }
            if !b {
                result += prints[prints.len() / 2]
            }
        }
        result
    } else {
        unreachable!()
    }
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let content = read_input(file_path);
    if let [page_order_rules, updates] = content.split("\n\n").collect::<Vec<&str>>()[..] {
        let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();
        for page_order in page_order_rules.lines() {
            if let [a, b] = page_order
                .splitn(2, "|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()[..]
            {
                ordering.entry(a).or_default().push(b)
            } else {
                unreachable!()
            }
        }
        let mut result = 0;
        for update in updates.lines() {
            let mut already_seen: Vec<u32> = Vec::new();
            let mut prints: Vec<u32> = update.split(",").map(|x| x.parse().unwrap()).collect();
            let mut b = false;
            for print in prints.iter() {
                if !ordering.get(print).map_or(true, |y| {
                    y.iter()
                        .all(|should_be_after| !already_seen.contains(should_be_after))
                }) {
                    b = true;
                    break;
                }
                already_seen.push(*print);
            }
            if b {
                let mut ordering = ordering.clone();
                let mut ordered_prints: Vec<u32> = Vec::new();
                let mut i = 0;
                while !prints.is_empty() {
                    let current = prints[i];
                    let mut can_place = true;
                    for (&a, xs) in ordering.iter() {
                        if prints.contains(&a) && a != current && xs.contains(&current) {
                            can_place = false;
                            break;
                        }
                    }
                    if can_place {
                        ordered_prints.push(current);
                        prints.remove(i);
                        for (_, order) in &mut ordering {
                            if let Some(i) = order.iter().position(|&x| x == current) {
                                order.remove(i);
                            }
                        }
                        i = 0;
                    } else {
                        i += 1;
                        if i >= prints.len() { i = 0 }
                    }
                }
                result += ordered_prints[ordered_prints.len() / 2];
            }
        }
        result
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 143);
        assert_eq!(part1(FILE_PATH), 5166);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 123);
        assert_eq!(part2(FILE_PATH), 4679);
    }
}
