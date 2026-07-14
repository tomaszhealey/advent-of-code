use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("Part 1: {}", Benchmark::new(|| part1(&input)));
    println!("Part 2: {}", Benchmark::new(|| part2(&input)));
}

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| {
        let mut triangle: Vec<usize> = l
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        triangle.sort();
        if triangle[0] + triangle[1] > triangle[2] {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> usize {
    let mut real_triangles = 0;
    let mut next = Vec::new();
    for line in input.lines() {
        next.push(line);
        if next.len() == 3 {
            let triple = next
                .iter()
                .map(|l| l.split_whitespace().map(|n| n.parse::<usize>().unwrap()));
            let mut triangles = [[0; 3]; 3];
            for (i, line) in triple.enumerate() {
                for (j, side) in line.enumerate() {
                    triangles[j][i] = side;
                }
            }
            for mut triangle in triangles {
                triangle.sort();
                if triangle[0] + triangle[1] > triangle[2] {
                    real_triangles += 1;
                }
            }
            next.clear();
        }
    }

    real_triangles
}
