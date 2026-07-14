use primitive_benchmarker::Benchmark;
use std::{collections::HashSet, env, fs};
use Direction::{East, North, South, West};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input
        .trim_end()
        .split(", ")
        .map(|l| (&l[0..1], l[1..].parse::<isize>().unwrap()));

    let (part1, part2) = Benchmark::elapsed("", || run(input));
    println!("Part 1: {part1}\nPart 2: {part2}");
}

enum Direction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
}

fn run<'a, I: Iterator<Item = (&'a str, isize)>>(input: I) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0isize;
    let dirs = [North(1), East(1), South(-1), West(-1)];
    let mut visited = HashSet::new();
    let mut part2 = -1;

    for (turn, count) in input {
        dir = match turn {
            "L" => (dir - 1).rem_euclid(4),
            _ => (dir + 1) % 4,
        };

        match dirs[dir as usize] {
            North(m) | South(m) => {
                let n = m * count;
                part2 = update_visited(part2, y, n + y, x, &mut visited, |constant, i| {
                    (constant, i)
                });
                y += n;
            }
            East(m) | West(m) => {
                let n = m * count;
                part2 = update_visited(part2, x, x + n, y, &mut visited, |constant, i| {
                    (i, constant)
                });
                x += n;
            }
        }
    }

    (x.abs() + y.abs(), part2)
}

fn update_visited(
    part2: isize,
    bound1: isize,
    bound2: isize,
    constant: isize,
    visited: &mut HashSet<(isize, isize)>,
    build_point: impl Fn(isize, isize) -> (isize, isize),
) -> isize {
    if part2 == -1 {
        let range = if bound1 < bound2 {
            bound1..bound2
        } else {
            (bound2 + 1)..(bound1 + 1)
        };

        for i in range {
            let point = build_point(constant, i);
            if visited.contains(&point) {
                return point.0.abs() + point.1.abs();
            } else {
                visited.insert(point);
            }
        }
    }
    part2
}
