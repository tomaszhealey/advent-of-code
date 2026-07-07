use benchmarker::benchmark_return;
use std::{env, fs};
use Instruction::{Fly, Rest};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let mut reindeer = parse(input.trim_end());

    let ((part1, part2), elapsed) = benchmark_return(|| run(&mut reindeer));
    println!("Part 1: {part1}\nPart 2: {part2}\n{elapsed}");
}

struct Reindeer {
    speed: usize,
    flight_time: usize,
    rest_time: usize,
    instruction: Instruction,
    distance: usize,
    points: usize,
}

enum Instruction {
    Fly { rem: usize },
    Rest { rem: usize },
}

fn as_usize(input: &str) -> usize {
    input.parse().unwrap()
}

fn parse(input: &str) -> Vec<Reindeer> {
    let mut reindeer = Vec::new();
    for line in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
        let flight_time = as_usize(line[6]);
        reindeer.push(Reindeer {
            speed: as_usize(line[3]),
            flight_time,
            rest_time: as_usize(line[13]),
            instruction: Fly { rem: flight_time },
            distance: 0,
            points: 0,
        });
    }

    reindeer
}

fn run(reindeer: &mut Vec<Reindeer>) -> (usize, usize) {
    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            match r.instruction {
                Fly { rem } => {
                    r.distance += r.speed;
                    if rem > 1 {
                        r.instruction = Fly { rem: rem - 1 };
                    } else {
                        r.instruction = Rest { rem: r.rest_time }
                    }
                }
                Rest { rem } => {
                    if rem > 1 {
                        r.instruction = Rest { rem: rem - 1 };
                    } else {
                        r.instruction = Fly { rem: r.flight_time };
                    }
                }
            }
        }

        award_points(reindeer);
    }

    (
        reindeer
            .iter()
            .fold(0, |acc, x| if x.distance > acc { x.distance } else { acc }),
        reindeer
            .iter()
            .fold(0, |acc, x| if x.points > acc { x.points } else { acc }),
    )
}

fn award_points(reindeer: &mut Vec<Reindeer>) {
    let mut best = 0;
    for r in reindeer.iter() {
        if best < r.distance {
            best = r.distance;
        }
    }

    for r in reindeer.iter_mut() {
        if best == r.distance {
            r.points += 1;
        }
    }
}
