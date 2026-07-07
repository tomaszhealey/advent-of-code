use benchmarker::benchmark_return;
use itertools::Itertools;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let containers = input
        .trim_end()
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let ((part1, part2), elapsed) = benchmark_return(|| run(&containers));
    println!("{elapsed}\nPart 1: {part1}\nPart 2: {part2}");
}

fn run(containers: &Vec<usize>) -> (usize, usize) {
    let mut total = 0;
    let mut part2 = 0;
    for i in 1..containers.len() {
        let combos = containers.into_iter().combinations(i);
        let subtotal = combos.fold(0, |acc, x| {
            if x.iter().fold(0, |acc, y| acc + *y) == 150 {
                acc + 1
            } else {
                acc
            }
        });

        if part2 == 0 && subtotal > 0 {
            part2 = subtotal
        }

        total += subtotal;
    }

    (total, part2)
}
