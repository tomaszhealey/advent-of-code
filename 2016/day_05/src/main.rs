use md5::{Digest, Md5};
use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();
    let hex: Vec<char> = "0123456789abcdef".chars().collect();

    println!("Part 1: {}", Benchmark::new(|| part1(&input, &hex)));
    println!("Part 2: {}", Benchmark::new(|| part2(&input, &hex)));
}

fn part1(input: &str, hex: &Vec<char>) -> String {
    let mut password = String::new();
    let mut number = 0;
    loop {
        let string = input.to_string() + &number.to_string();
        let hash = Md5::digest(string.as_bytes());

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            password.push(hex[hash[2] as usize]);
            if password.len() == 8 {
                break;
            }
        }

        number += 1;
    }
    password
}

fn part2(input: &str, hex: &Vec<char>) -> String {
    let mut password = vec![' '; 8];
    let mut number = 0;
    loop {
        let string = input.to_string() + &number.to_string();
        let hash = Md5::digest(string.as_bytes());

        if hash[0] == 0 && hash[1] == 0 && hash[2] < 8 && password[hash[2] as usize] == ' ' {
            password[hash[2] as usize] = hex[((hash[3] & 0b11110000) >> 4) as usize];
            if !password.contains(&' ') {
                break;
            }
        }

        number += 1;
    }
    password.iter().collect::<String>()
}
