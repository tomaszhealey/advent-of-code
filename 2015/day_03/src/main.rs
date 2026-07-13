use primitive_benchmarker::Benchmark;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");

    println!("Part 1: {}", Benchmark::new(|| part1(&input)));
    println!("Part 2: {}", Benchmark::new(|| part2(&input)));
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Vector2 {
    x: isize,
    y: isize,
}

fn part1(input: &str) -> usize {
    let mut pos = Vector2 { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert((pos.x, pos.y));

    for char in input.chars() {
        match char {
            '>' => pos.x += 1,
            '^' => pos.y += 1,
            '<' => pos.x -= 1,
            'v' => pos.y -= 1,
            _ => continue,
        };
        visited.insert((pos.x, pos.y));
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut santa_pos = Vector2 { x: 0, y: 0 };
    let mut robot_pos = Vector2 { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let mut flag = false;
    visited.insert((0, 0));

    for char in input.chars() {
        let pos = if flag { &mut santa_pos } else { &mut robot_pos };
        match char {
            '>' => pos.x += 1,
            '^' => pos.y += 1,
            '<' => pos.x -= 1,
            'v' => pos.y -= 1,
            _ => continue,
        };

        flag = !flag;
        visited.insert((pos.x, pos.y));
    }

    visited.len()
}
