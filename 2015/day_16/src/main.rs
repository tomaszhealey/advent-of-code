use benchmarker::benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let aunties = parse(input.trim_end());

    let target = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    println!("Part 1: {}", benchmark(|| part1(&aunties, &target)));
    println!("Part 2: {}", benchmark(|| part2(&aunties, &target)));
}

fn parse(input: &str) -> Vec<HashMap<&str, usize>> {
    let mut out = Vec::new();
    for line in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
        let mut aunty = HashMap::new();
        let mut i = 2;
        while i <= 6 {
            aunty.insert(
                line[i].strip_suffix(":").unwrap(),
                line[i + 1]
                    .strip_suffix(",")
                    .unwrap_or(line[i + 1])
                    .parse()
                    .unwrap(),
            );
            i += 2;
        }
        out.push(aunty);
    }

    out
}

fn part1(aunties: &Vec<HashMap<&str, usize>>, target: &HashMap<&str, usize>) -> usize {
    'outer: for (index, aunty) in aunties.iter().enumerate() {
        for key in aunty.keys() {
            if aunty[key] != target[key] {
                continue 'outer;
            }
        }
        return index + 1;
    }
    panic!("No aunty found in part 1.");
}

fn part2(aunties: &Vec<HashMap<&str, usize>>, target: &HashMap<&str, usize>) -> usize {
    'outer: for (index, aunty) in aunties.iter().enumerate() {
        for key in aunty.keys() {
            match *key {
                "cats" | "trees" => {
                    if aunty[key] <= target[key] {
                        continue 'outer;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if aunty[key] >= target[key] {
                        continue 'outer;
                    }
                }
                _ => {
                    if aunty[key] != target[key] {
                        continue 'outer;
                    }
                }
            }
        }
        return index + 1;
    }

    panic!("No aunty found in part 2.");
}
