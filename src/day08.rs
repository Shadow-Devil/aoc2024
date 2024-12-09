use crate::read_lines;
use crate::util::read_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day08.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day08_sample.txt";

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub(crate) fn part1(file_path: &str) -> u32 {
    let (antenna_groups, max) = parse(file_path);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, antennas) in antenna_groups.iter() {
        for (&a, &b) in antennas.iter().combinations(2).map(|p| (p[0], p[1])) {
            let diff = a - b;
            add_antinode(&mut antinodes, &max, a + diff);
            add_antinode(&mut antinodes, &max, b - diff);
        }
    }

    antinodes.len() as u32
}

fn parse(file_path: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut antenna_groups: HashMap<char, Vec<Point>> = HashMap::new();
    let mut max = Point { x: 0, y: 0 };

    for (y, line) in read_lines!(file_path).enumerate() {
        max.y += 1;
        max.x = line.len() as i32;
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antenna_groups.entry(char).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    (antenna_groups, max)
}

fn add_antinode(antinodes: &mut HashSet<Point>, max: &Point, c: Point) {
    if in_grid(&c, max) {
        antinodes.insert(c);
    }
}

fn in_grid(p: &Point, max: &Point) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < max.x && p.y < max.y
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let (antenna_groups, max) = parse(file_path);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_, antennas) in antenna_groups.iter() {
        for (&a, &b) in antennas.iter().combinations(2).map(|p| (p[0], p[1])) {
            add_antinode(&mut antinodes, &max, a);
            add_antinode(&mut antinodes, &max, b);
            let diff = a - b;
            let mut tmp = a + diff;
            while in_grid(&tmp, &max) {
                add_antinode(&mut antinodes, &max, tmp);
                tmp = tmp + diff;
            }
            tmp = b - diff;
            while in_grid(&tmp, &max) {
                add_antinode(&mut antinodes, &max, tmp);
                tmp = tmp - diff;
            }
        }
    }
    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 14);
        assert_eq!(part1(FILE_PATH), 409);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 34);
        assert_eq!(part2(FILE_PATH), 1308);
    }
}
