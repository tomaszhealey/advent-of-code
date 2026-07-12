use benchmarker::benchmark;
use itertools::Itertools;
use std::{env, fs, usize};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let weights: Vec<usize> = input
        .trim_end()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    println!("Part 1: {}", benchmark(|| run(&weights, 3).unwrap()));
    println!("Part 2: {}", benchmark(|| run(&weights, 4).unwrap()));
}

fn run(weights: &Vec<usize>, groups: usize) -> Option<usize> {
    let target = weights.iter().sum::<usize>() / groups;
    for i in 2..=weights.len() {
        let mut valid = weights
            .clone()
            .into_iter()
            .permutations(i)
            .filter(|p| p.iter().sum::<usize>() == target);
        if let Some(p) = valid.next() {
            return Some(p.iter().product::<usize>());
        }
    }

    None
}
