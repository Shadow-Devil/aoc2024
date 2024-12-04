use crate::read_lines;
use crate::util::{read_input, Countable};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::Iterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day04.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day04_sample.txt";

lazy_static! {
    static ref MAPPING: HashMap<char, u32> =
        HashMap::from([('X', 1), ('M', 2), ('A', 3), ('S', 4),]);
}

pub(crate) fn part1(file_path: &str) -> u32 {
    let grid = parse_grid(file_path);
    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            result += find_word(x, y, &grid);
        }
    }
    result
}

fn parse_grid(file_path: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for (x, line) in read_lines!(file_path).enumerate() {
        grid.push(Vec::new());
        for (_y, c) in line.chars().enumerate() {
            grid[x].push(*MAPPING.get(&c).unwrap_or(&0));
        }
    }
    grid
}

fn find_word(x: usize, y: usize, grid: &[Vec<u32>]) -> u32 {
    if grid[x][y] != 1 {
        return 0;
    }

    let offsets: [[(i32, i32); 3]; 8] = [
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(1, -1), (2, -2), (3, -3)],
        [(-1, -1), (-2, -2), (-3, -3)],
    ];

    offsets.iter().count_all(|[(x1, y1), (x2, y2), (x3, y3)]| {
        is_correct(
            grid.get((x as i32 + x1) as usize)
                .and_then(|xs| xs.get((y as i32 + y1) as usize)),
            grid.get((x as i32 + x2) as usize)
                .and_then(|xs| xs.get((y as i32 + y2) as usize)),
            grid.get((x as i32 + x3) as usize)
                .and_then(|xs| xs.get((y as i32 + y3) as usize)),
        )
    }) as u32
}

fn is_correct(a: Option<&u32>, b: Option<&u32>, c: Option<&u32>) -> bool {
    a.is_some_and(|&x| x == 2) && b.is_some_and(|&x| x == 3) && c.is_some_and(|&x| x == 4)
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let grid = parse_grid(file_path);
    let mut result = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let tmp = find_word_part2(x, y, &grid);
            if tmp != 0 {
                //println!("{} {}", x, y);
            }
            result += tmp;
        }
    }
    result
}

fn find_word_part2(x: usize, y: usize, grid: &[Vec<u32>]) -> u32 {
    if grid[x][y] != 3 {
        return 0;
    }

    let offsets: [[(i32, i32); 4]; 1] = [[(1, 1), (1, -1), (-1, -1), (-1, 1)]];

    offsets
        .iter()
        .count_all(|[(x1, y1), (x2, y2), (x3, y3), (x4, y4)]| {
            is_correct_part2(
                grid.get((x as i32 + x1) as usize)
                    .and_then(|xs| xs.get((y as i32 + y1) as usize)),
                grid.get((x as i32 + x2) as usize)
                    .and_then(|xs| xs.get((y as i32 + y2) as usize)),
                grid.get((x as i32 + x3) as usize)
                    .and_then(|xs| xs.get((y as i32 + y3) as usize)),
                grid.get((x as i32 + x4) as usize)
                    .and_then(|xs| xs.get((y as i32 + y4) as usize)),
            )
        }) as u32
}

fn is_correct_part2(a: Option<&u32>, b: Option<&u32>, c: Option<&u32>, d: Option<&u32>) -> bool {
    match (a, b, c, d) {
        (Some(&a), Some(&b), Some(&c), Some(&d)) =>
            ((a == 2 && c == 4) || (a == 4 && c == 2)) &&
            ((b == 2 && d == 4) || (b == 4 && d == 2)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 18);
        assert_eq!(part1(FILE_PATH), 2551);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 9);
        assert_eq!(part2(FILE_PATH), 1985);
    }
}
