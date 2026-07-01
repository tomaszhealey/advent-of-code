use benchmarker::benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("inputs/input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("Part 1: {}", benchmark(|| part1(&input)));
    println!("Part 2: {}", benchmark(|| part2(&input)));
}

fn part1(input: &str) -> usize {
    input
        .chars()
        .map(|c| if c == '(' { 1isize } else { -1isize })
        .sum::<isize>() as usize
}

fn part2(input: &str) -> usize {
    let values = input
        .chars()
        .map(|c| if c == '(' { 1isize } else { -1isize });

    let mut floor = 0isize;
    for (pos, value) in values.enumerate() {
        floor += value;
        if floor < 0 {
            return (pos + 1) as usize;
        }
    }

    0usize
}
