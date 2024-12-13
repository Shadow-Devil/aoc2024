use rayon::iter::ParallelIterator;
use crate::util::{read_input, Point};
use std::collections::HashSet;
use std::iter::Iterator;
use rayon::iter::IntoParallelRefIterator;

#[allow(unused)]
pub(crate) const FILE_PATH: &str = "input/day06.txt";
#[allow(unused)]
pub(crate) const FILE_PATH_SAMPLE: &str = "input/day06_sample.txt";


#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct PointWithDir {
    point: Point,
    direction: Direction
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub(crate) fn part1(file_path: &str) -> usize {
    let (obstacles, start, size) = parse(file_path);

    run_simulation(&obstacles, start, &size).unwrap().len()
}

fn parse(file_path: &str) -> (Vec<Point>, PointWithDir, Point) {
    let content = read_input(file_path);

    let mut obstacles = Vec::new();
    let mut start = PointWithDir { point: Point { x: -1, y: -1 }, direction: Direction::North };

    let size = Point { x: content.lines().count() as i32, y: content.lines().last().unwrap().len() as i32 };

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    let point = Point { x: x as i32, y: y as i32 };
                    obstacles.insert(obstacles.binary_search(&point).unwrap_err(), point);
                }
                '^' => {
                    start.point.x = x as i32;
                    start.point.y = y as i32;
                }
                _ => {}
            }
        }
    }
    assert!(start.point.x != -1 && start.point.y != -1);
    (obstacles, start, size)
}

fn run_simulation(obstacles: &Vec<Point>, mut guard: PointWithDir, size: &Point) -> Option<HashSet<Point>> {
    assert!(!obstacles.iter().any(|o| o.x == guard.point.x && o.y == guard.point.y ));
    let mut visited = HashSet::new();
    while guard.point.x >= 0 && guard.point.x < size.x && guard.point.y >= 0 && guard.point.y < size.y {
        if visited.contains(&guard) {
            return None;
        }
        match guard.direction {
            Direction::North => {
                match obstacles.iter().rev().find(|p| p.x == guard.point.x && p.y < guard.point.y) {
                    Some(Point { x: _, y: y_obs }) => {
                        assert!(!obstacles.iter().any(|p| p.x == guard.point.x && p.y < guard.point.y && p.y > *y_obs));
                        let border = *y_obs + 1;
                        for y1 in border..=guard.point.y {
                            visited.insert(PointWithDir { point: Point { x: guard.point.x, y: y1 }, direction: guard.direction });
                        }
                        guard.point.y = border;
                        guard.direction = Direction::East;
                    }
                    None => {
                        for y1 in 0..=guard.point.y {
                            visited.insert(PointWithDir { point: Point { x: guard.point.x, y: y1 }, direction: guard.direction });
                        }
                        guard.point.y = -1;
                    }
                }
            }
            Direction::East => {
                match obstacles.iter().find(|p| p.x > guard.point.x && p.y == guard.point.y) {
                    Some(Point { x: x_obs, y: _ }) => {
                        assert!(!obstacles.iter().any(|p| p.y == guard.point.y && p.x > guard.point.x && p.x < *x_obs));
                        let border = *x_obs - 1;
                        for x1 in guard.point.x..=border {
                            visited.insert(PointWithDir { point: Point { x: x1, y: guard.point.y }, direction: guard.direction });
                        }
                        guard.point.x = border;
                        guard.direction = Direction::South;
                    }
                    None => {
                        for x1 in 0..guard.point.x {
                            visited.insert(PointWithDir { point: Point { x: x1, y: guard.point.y }, direction: guard.direction });
                        }
                        guard.point.x = -1;
                    }
                }
            }
            Direction::South => {
                match obstacles.iter().find(|p| p.x == guard.point.x && p.y > guard.point.y) {
                    Some(Point { x: _, y: y_obs }) => {
                        assert!(!obstacles.iter().any(|p| p.x == guard.point.x && p.y > guard.point.y && p.y < *y_obs));
                        let border = *y_obs - 1;
                        for y1 in guard.point.y..=border {
                            visited.insert(PointWithDir { point: Point { x: guard.point.x, y: y1 }, direction: guard.direction });
                        }
                        guard.point.y = border;
                        guard.direction = Direction::West;
                    }
                    None => {
                        for y1 in guard.point.y..size.y {
                            visited.insert(PointWithDir { point: Point { x: guard.point.x, y: y1 }, direction: guard.direction });
                        }
                        guard.point.y = size.y;
                    }
                }
            }
            Direction::West => {
                match obstacles.iter().rev().find(|p| p.x < guard.point.x && p.y == guard.point.y) {
                    Some(Point { x: x_obs, y: _ }) => {
                        assert!(!obstacles.iter().any(|p| p.y == guard.point.y && p.x < guard.point.x && p.x > *x_obs));
                        let border = *x_obs + 1;
                        for x1 in border..guard.point.x {
                            visited.insert(PointWithDir { point: Point { x: x1, y: guard.point.y }, direction: guard.direction });
                        }
                        guard.point.x = border;
                        guard.direction = Direction::North;
                    }
                    None => {
                        for x1 in guard.point.x..size.x {
                            visited.insert(PointWithDir { point: Point { x: x1, y: guard.point.y }, direction: guard.direction });
                        }
                        guard.point.x = size.x;
                    }
                }
            }
        }
    }
    Some(visited.iter().map(|x|x.point).collect())
}

pub(crate) fn part2(file_path: &str) -> u32 {
    let (obstacles, start, size) = parse(file_path);

    let potential_obstacles = run_simulation(&obstacles, start.clone(), &size).unwrap();


    potential_obstacles.par_iter().filter(|o| !(o.x == start.point.x && o.y == start.point.y)).map(|o| {
        let mut obs = obstacles.clone();
        obs.insert(obs.binary_search(o).unwrap_err(), *o);
        run_simulation(&obs, start.clone(), &size).is_none() as u32
    }).sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part1() {
        assert_eq!(part1(FILE_PATH_SAMPLE), 41);
        assert_eq!(part1(FILE_PATH), 5409);
    }
    #[test]
    fn check_part2() {
        assert_eq!(part2(FILE_PATH_SAMPLE), 6);
        assert_eq!(part2(FILE_PATH), 2022);
    }
}
