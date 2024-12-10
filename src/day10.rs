use crate::read_lines;
use crate::util::{read_input, Point};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day10.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day10_sample.txt";

pub(crate) fn part1(file_path: &str) -> usize {
    let (points, start_points) = parse(file_path);
    let mut result = 0;

    for p in start_points {
        let mut reachable_top: HashSet<Point> = HashSet::new();
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: BinaryHeap<Point> = BinaryHeap::new();
        queue.push(p);

        while let Some(p) = queue.pop() {
            for neighbor in neighbors(&points, &p) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push(neighbor);
                }
                if points[&neighbor] == 9 {
                    reachable_top.insert(neighbor);
                }
            }
        }
        result += reachable_top.len();
    }

    result
}

fn parse(file_path: &str) -> (HashMap<Point, u32>, HashSet<Point>) {
    let mut points = HashMap::new();
    let mut start_points = HashSet::new();
    for (y, line) in read_lines!(file_path).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let num = ch.to_digit(10).unwrap();
            let p = Point {
                x: x as i32,
                y: y as i32,
            };
            points.insert(p, num);
            if num == 0 {
                start_points.insert(p);
            }
        }
    }
    (points, start_points)
}

fn neighbors(points: &HashMap<Point, u32>, p: &Point) -> Vec<Point> {
    let &Point { x, y } = p;
    let height = points[p];
    [
        Point { x: x + 1, y },
        Point { x: x - 1, y },
        Point { x, y: y - 1 },
        Point { x, y: y + 1 },
    ]
    .iter()
    .filter(|p| points.get(p).map(|&v| v == height + 1).unwrap_or(false))
    .copied()
    .collect()
}

pub(crate) fn part2(file_path: &str) -> u64 {
    let (points, start_points) = parse(file_path);
    let mut result = 0;

    for p in start_points {
        let mut paths = 0;
        let mut queue: BinaryHeap<Point> = BinaryHeap::new();
        queue.push(p);

        while let Some(p) = queue.pop() {
            for neighbor in neighbors(&points, &p) {
                queue.push(neighbor);
                if points[&neighbor] == 9 {
                    paths += 1;
                }
            }
        }
        println!("{:?}", paths);
        result += paths;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 36);
        assert_eq!(part1(FILE_PATH), 607);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 81);
        assert_eq!(part2(FILE_PATH), 1384);
    }
}
