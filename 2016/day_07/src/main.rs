use fancy_regex::Regex;
use primitive_benchmarker::Benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();
    let get_hypernets = Regex::new(r"\[.+?\]").unwrap();

    println!(
        "Part 1: {}",
        Benchmark::new(|| part1(&input, &get_hypernets))
    );

    println!(
        "Part 2: {}",
        Benchmark::new(|| part2(&input, &get_hypernets))
    );
}

fn part1(input: &str, get_hypernets: &Regex) -> usize {
    let mut num = 0;
    let get_tls = Regex::new(r"(.)(.)(?<!\1\1)\2\1").unwrap();
    'outer: for line in input.lines() {
        let hypernets = get_hypernets.captures_iter(line);
        let mut line = line.to_string();
        for s in hypernets {
            let s = s.unwrap().get(0).unwrap().as_str();
            if let Ok(Some(_)) = get_tls.captures(s) {
                continue 'outer;
            }
            line = line.replacen(s, " ", 1);
        }

        if let Ok(Some(_)) = get_tls.captures(&line) {
            num += 1;
        }
    }

    num
}

fn part2(input: &str, get_hypernets: &Regex) -> usize {
    let mut num = 0;
    let get_aba = Regex::new(r"(?=((.)(.)(?<!\2\2)\2))").unwrap();
    for line in input.lines() {
        let hypernets = get_hypernets
            .captures_iter(line)
            .map(|s| s.unwrap().get(0).unwrap().as_str())
            .collect::<Vec<_>>();
        let mut line = line.to_string();
        for s in hypernets.iter() {
            line = line.replacen(s, " ", 1);
        }

        let babs = get_aba
            .captures_iter(&line)
            .map(|x| {
                let x = x.unwrap().get(1).unwrap().as_str();
                let y = String::new() + &x[1..2] + &x[0..1] + &x[1..2];
                y
            })
            .collect::<Vec<_>>();

        for s in hypernets {
            if babs.iter().any(|x| s.contains(x)) {
                num += 1;
                break;
            }
        }
    }

    num
}
