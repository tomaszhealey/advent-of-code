use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("Part 1 : {}", Benchmark::new(|| decompress(input, true)));
    println!("Part 2 : {}", Benchmark::new(|| decompress(input, false)));
}

fn decompress(data: &str, is_part1: bool) -> usize {
    let mut length = 0;
    let chars = data.chars().collect::<Vec<_>>();
    let mut pntr = 0;

    while pntr < data.len() {
        if chars[pntr] == '(' {
            let s = data[pntr + 1..].split_once(')').unwrap();
            pntr += s.0.len() + 2;
            let nums =
                s.0.split('x')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

            if is_part1 {
                length += nums[0] * nums[1];
            } else {
                length += decompress(&data[pntr..pntr + nums[0]], false) * nums[1];
            }
            pntr += nums[0];
        } else {
            pntr += 1;
            length += 1;
        }
    }

    length
}
