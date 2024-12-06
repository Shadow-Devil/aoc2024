use crate::util::read_input;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day05.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day05_sample.txt";

pub(crate) fn part1(file_path: &str) -> u32 {
    if let [page_order_rules, updates] =
        read_input(file_path).split("\n\n").collect::<Vec<&str>>()[..]
    {
        let ordering = parse_ordering(page_order_rules);
        updates
            .lines()
            .map(|update| {
                let prints: Vec<u32> = update.split(",").map(|x| x.parse().unwrap()).collect();
                if is_ordered(&ordering, &prints) {
                    prints[prints.len() / 2]
                } else {
                    0
                }
            })
            .sum()
    } else {
        unreachable!()
    }
}

fn is_ordered(ordering: &HashMap<u32, HashSet<u32>>, prints: &[u32]) -> bool {
    let mut already_seen = HashSet::with_capacity(prints.len());
    for print in prints.iter() {
        if !ordering.get(print).map_or(true, |y| {
            y.iter()
                .all(|should_be_after| !already_seen.contains(should_be_after))
        }) {
            return false;
        }
        already_seen.insert(*print);
    }
    true
}

pub(crate) fn part2(file_path: &str) -> u32 {
    if let [page_order_rules, updates] =
        read_input(file_path).split("\n\n").collect::<Vec<&str>>()[..]
    {
        let ordering = parse_ordering(page_order_rules);
        let mut result = 0;
        for update in updates.lines() {
            let prints: Vec<u32> = update.split(",").map(|x| x.parse().unwrap()).collect();

            if is_ordered(&ordering, &prints) {
                continue;
            }

            let ordered_prints = fix_ordering(&ordering, prints);
            result += ordered_prints[ordered_prints.len() / 2];
        }
        result
    } else {
        unreachable!()
    }
}

fn fix_ordering(ordering: &HashMap<u32, HashSet<u32>>, mut prints: Vec<u32>) -> Vec<u32> {
    let mut ordered_prints: Vec<u32> = Vec::with_capacity(prints.len());
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
            i = 0;
        } else {
            i = (i + 1) % prints.len();
        }
    }
    ordered_prints
}

fn parse_ordering(page_order_rules: &str) -> HashMap<u32, HashSet<u32>> {
    let mut ordering: HashMap<u32, HashSet<u32>> = HashMap::new();
    for page_order in page_order_rules.lines() {
        if let [a, b] = page_order
            .splitn(2, "|")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()[..]
        {
            ordering.entry(a).or_default().insert(b);
        } else {
            unreachable!()
        }
    }
    ordering
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
