use primitive_benchmarker::Benchmark;
use regex::Regex;
use serde_json::Value;
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

fn part1(input: &str) -> isize {
    let re = Regex::new(r"\d+|-\d+").unwrap();
    re.find_iter(input)
        .map(|x| x.as_str().parse::<isize>().unwrap())
        .sum::<isize>()
}

fn part2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    sum(&v)
}

fn sum(val: &Value) -> i64 {
    if let Value::Object(obj) = val {
        if obj.values().any(|val| val == "red") {
            return 0;
        } else {
            return obj.values().fold(0, |acc, x| acc + process_val(x));
        }
    } else if let Value::Array(arr) = val {
        arr.iter().fold(0i64, |acc, x| acc + process_val(x))
    } else {
        0
    }
}

fn process_val(v: &Value) -> i64 {
    match v {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(_) | Value::Object(_) => sum(v),
        _ => 0,
    }
}
