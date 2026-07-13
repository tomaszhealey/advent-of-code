use primitive_benchmarker::Benchmark;
use std::{env, fmt::Display, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!("{}", Benchmark::new(|| run(&input)));
}

struct Output {
    paper: usize,
    ribbon: usize,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part 1: {}, Part 2: {}", self.paper, self.ribbon)
    }
}

fn run(input: &str) -> Output {
    let outputs = input.lines().map(|x| {
        let mut dims: Vec<usize> = x.split("x").map(|y| y.parse().unwrap()).collect();
        dims.sort();

        Output {
            paper: 3 * (dims[0] * dims[1]) + 2 * (dims[0] * dims[2] + dims[1] * dims[2]),
            ribbon: 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2],
        }
    });

    let mut output = Output {
        paper: 0,
        ribbon: 0,
    };

    for value in outputs {
        output.paper += value.paper;
        output.ribbon += value.ribbon;
    }

    output
}
