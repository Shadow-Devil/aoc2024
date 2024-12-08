use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;

fn main() {
    let now_all = Instant::now();
    let mut now = Instant::now();
    println!("Day01 Part1: {} ({}ms)", day01::part1(day01::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day01 Part2: {} ({}ms)", day01::part2(day01::FILE_PATH), now.elapsed().as_millis());

    now = Instant::now();
    println!("Day02 Part1: {} ({}ms)", day02::part1(day02::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day02 Part2: {} ({}ms)", day02::part2(day02::FILE_PATH), now.elapsed().as_millis());
    
    now = Instant::now();
    println!("Day03 Part1: {} ({}ms)", day03::part1(day03::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day03 Part2: {} ({}ms)", day03::part2(day03::FILE_PATH), now.elapsed().as_millis());
    
    now = Instant::now();
    println!("Day04 Part1: {} ({}ms)", day04::part1(day04::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day04 Part2: {} ({}ms)", day04::part2(day04::FILE_PATH), now.elapsed().as_millis());
    
    now = Instant::now();
    println!("Day05 Part1: {} ({}ms)", day05::part1(day05::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day05 Part2: {} ({}ms)", day05::part2(day05::FILE_PATH), now.elapsed().as_millis());

    now = Instant::now();
    println!("Day06 Part1: {} ({}ms)", day06::part1(day06::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    //Slow (20 sec in dev run, 2 sec in release)
    //println!("Day06 Part2: {} ({}ms)", day06::part2(day06::FILE_PATH), now.elapsed().as_millis());
    
    now = Instant::now();
    println!("Day07 Part1: {} ({}ms)", day07::part1(day07::FILE_PATH), now.elapsed().as_millis());
    now = Instant::now();
    println!("Day07 Part2: {} ({}ms)", day07::part2(day07::FILE_PATH), now.elapsed().as_millis());
        
    
    println!("Overall {}ms", now_all.elapsed().as_millis())
}
