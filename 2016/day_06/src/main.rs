use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(&input, 0, |stored, new| stored < new))
    );

    println!(
        "Part 2: {}",
        Benchmark::new(|| run(&input, usize::MAX, |stored, new| stored > new))
    );
}

fn run(input: &str, default: usize, compare: impl Fn(usize, usize) -> bool) -> String {
    let mut frequencies = vec![HashMap::new(); 8];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let freq = frequencies[i].entry(c).or_insert(0usize);
            *freq += 1;
        }
    }

    let mut msg = String::new();
    for map in frequencies {
        let mut greatest = (' ', default);
        for (&c, &i) in map.iter() {
            if compare(greatest.1, i) {
                greatest = (c, i);
            }
        }
        msg.push(greatest.0);
    }
    msg
}
