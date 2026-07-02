use benchmarker::benchmark;
use md5::{Digest, Md5};
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

fn part1(index: &str) -> usize {
    let mut number: usize = 0;
    loop {
        let string = index.to_string() + &number.to_string();
        let hash = Md5::digest(string.as_bytes());

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return number;
        }

        number += 1;
    }
}

fn part2(index: &str) -> usize {
    let mut number: usize = 0;
    loop {
        let string = index.to_string() + &number.to_string();
        let hash = Md5::digest(string.as_bytes());

        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return number;
        }

        number += 1;
    }
}
