use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("Part 1: {}", Benchmark::new(|| solve(input, 40)));
    println!("Part 2: {}", Benchmark::new(|| solve(input, 50)));
}

fn solve(input: &str, repeat: usize) -> usize {
    let mut num = String::from(input);

    for _ in 0..repeat {
        let mut chars = num.chars();
        let mut count = 1usize;
        let mut digit: char = chars.next().unwrap();
        let mut new = String::new();

        for c in chars {
            if c != digit {
                push_new(count, &digit, &mut new);
                count = 1;
                digit = c;
            } else {
                count += 1;
            }
        }

        push_new(count, &digit, &mut new);
        num = new;
    }

    num.to_string().len()
}

fn push_new(count: usize, digit: &char, new: &mut String) {
    new.push_str(&count.to_string());
    new.push(*digit);
}
