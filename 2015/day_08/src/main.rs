use benchmarker::benchmark;
use regex::Regex;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("Part 1: {}", benchmark(|| part1(&input)));
    println!("Part 2: {}", benchmark(|| part2(&input)));
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"(\\\W|\\x..)").unwrap();
    input
        .lines()
        .map(|x| 
            //"a" is arbitrary to represent the escape sequence and + 2 for the string quotes
            x.len() - re.replace_all(x, "a").len() + 2
        )
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"(?<x>\W)").unwrap();
    input.lines().map(|x| 
        re.find_iter(x).collect::<Vec<_>>().len() + 2
    )
    .sum::<usize>()
}
