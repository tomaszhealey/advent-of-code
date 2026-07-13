use itertools::Itertools;
use primitive_benchmarker::Benchmark;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end().parse::<usize>().unwrap();
    println!("Part 1: {}", Benchmark::new(|| part1(input)));
    println!("Part 2: {}", Benchmark::new(|| part2(input)));
}

fn get_prime_factors(mut num: usize) -> Vec<usize> {
    let mut factors = vec![];
    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }

    let mut d = 3;
    while d * d <= num {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        }
        d += 2;
    }

    if num > 1 {
        factors.push(num);
    }

    factors
}

fn part1(target: usize) -> usize {
    let mut i = 2;
    let mut presents = 0;
    let mut computed = HashSet::new();
    while presents < target {
        i += 2; // answer will be even
        presents = 0;
        let pset = get_prime_factors(i).into_iter().powerset();
        computed.clear();

        for set in pset {
            if computed.contains(&set) {
                continue;
            }
            presents += 10 * set.iter().product::<usize>();
            computed.insert(set);
        }
    }

    i
}

fn part2(target: usize) -> usize {
    let mut i = 1;
    let mut presents = 0;
    let mut computed = HashSet::new();
    while presents < target {
        i += 1;
        presents = 0;
        let pset = get_prime_factors(i).into_iter().powerset();
        computed.clear();

        for set in pset {
            if computed.contains(&set) {
                continue;
            }
            let p = set.iter().product::<usize>();
            computed.insert(set);
            if p * 50 < i {
                continue;
            }
            presents += 11 * p;
        }
    }

    i
}
