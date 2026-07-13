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
        Benchmark::new(|| input.lines().fold(0, |acc, x| acc + part1(x)))
    );

    println!(
        "Part 2: {}",
        Benchmark::new(|| input.lines().fold(0, |acc, x| {
            if check_for_pair(x) && check_for_split_pair(x) {
                acc + 1
            } else {
                acc
            }
        }))
    );
}

fn part1(x: &str) -> usize {
    let mut has_double = false;
    for left in 0..x.len() - 1 {
        let slice = &x[left..left + 2];
        if slice == "ab" || slice == "cd" || slice == "pq" || slice == "xy" {
            return 0;
        }

        let mut slice = slice.chars();
        if !has_double && slice.next() == slice.next() {
            has_double = true;
        }
    }

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let vowels_in_string = x
        .chars()
        .fold(0, |acc, x| if vowels.contains(&x) { acc + 1 } else { acc });

    if vowels_in_string > 2 && has_double {
        1
    } else {
        0
    }
}

//Part 2 functions
fn check_for_pair(x: &str) -> bool {
    let mut pairs = HashMap::new();
    for left in 0..x.len() - 1 {
        let pntr = pairs.entry(&x[left..left + 2]).or_insert(left);
        if left > *pntr + 1 {
            return true;
        }
    }

    false
}

fn check_for_split_pair(x: &str) -> bool {
    for left in 0..x.len() - 2 {
        let chars = &x[left..left + 3].chars().collect::<Vec<_>>();
        if chars[0] == chars[2] && chars[0] != chars[1] {
            return true;
        }
    }

    false
}
