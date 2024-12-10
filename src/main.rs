use std::fmt::Display;
use std::time::Instant;

mod util;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let now_all = Instant::now();
    print_day("Day01 Part1", day01::part1, day01::FILE_PATH);
    print_day("Day01 Part2", day01::part2, day01::FILE_PATH);
    print_day("Day02 Part1", day02::part1, day02::FILE_PATH);
    print_day("Day02 Part2", day02::part2, day02::FILE_PATH);
    print_day("Day03 Part1", day03::part1, day03::FILE_PATH);
    print_day("Day03 Part2", day03::part2, day03::FILE_PATH);
    print_day("Day04 Part1", day04::part1, day04::FILE_PATH);
    print_day("Day04 Part2", day04::part2, day04::FILE_PATH);
    print_day("Day05 Part1", day05::part1, day05::FILE_PATH);
    print_day("Day05 Part2", day05::part2, day05::FILE_PATH);
    print_day("Day06 Part1", day06::part1, day06::FILE_PATH);
    //print_day("Day06 Part2", day06::part1, day06::FILE_PATH);
    //print_day("Day07 Part1", day07::part1, day07::FILE_PATH);
    //print_day("Day07 Part2", day07::part2, day07::FILE_PATH);
    print_day("Day08 Part1", day08::part1, day08::FILE_PATH);
    print_day("Day08 Part2", day08::part2, day08::FILE_PATH);
    print_day("Day09 Part1", day09::part1, day09::FILE_PATH);
    //print_day("Day09 Part2", day09::part2, day09::FILE_PATH);
    print_day("Day10 Part1", day10::part1, day10::FILE_PATH);
    print_day("Day10 Part2", day10::part2, day10::FILE_PATH);
    
    println!("Overall {}ms", now_all.elapsed().as_millis())
}

fn print_day<R1: Display>(
    day_and_part: &str,
    part1: fn(&str) -> R1,
    file_path: &str,
) {
    let now = Instant::now();
    println!(
        "{}: {} ({}ms)",
        day_and_part,
        part1(file_path),
        now.elapsed().as_millis()
    );
}
