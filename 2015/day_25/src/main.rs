use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();
    let nums = input.split("row ").collect::<Vec<_>>()[1]
        .split(", column ")
        .collect::<Vec<_>>();
    let (row, col): (usize, usize) = (
        nums[0].parse().unwrap(),
        nums[1].strip_suffix(".").unwrap().parse().unwrap(),
    );

    println!("Part 1: {}", Benchmark::new(|| get_code(row, col)));
}

fn triangle_num(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn get_code(row: usize, col: usize) -> usize {
    let code_position = triangle_num(col + (row - 1)) - row;
    let mut code = 20151125;
    for _ in 0..code_position {
        code = (code * 252533) % 33554393;
    }

    code
}
