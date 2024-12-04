use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod util;

fn main() {
    let now = Instant::now();

    println!("Day01 Part1: {}", day01::part1(day01::FILE_PATH));
    println!("Day01 Part2: {}", day01::part2(day01::FILE_PATH));
    println!("Day02 Part1: {}", day02::part1(day02::FILE_PATH));
    println!("Day02 Part2: {}", day02::part2(day02::FILE_PATH));
    println!("Day03 Part1: {}", day03::part1(day03::FILE_PATH));
    println!("Day03 Part2: {}", day03::part2(day03::FILE_PATH));
    println!("Day04 Part1: {}", day04::part1(day04::FILE_PATH));
    println!("Day04 Part2: {}", day04::part2(day04::FILE_PATH));
    println!("Took {}ms", now.elapsed().as_millis());
}
