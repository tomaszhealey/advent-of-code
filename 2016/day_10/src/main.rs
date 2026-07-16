use primitive_benchmarker::Benchmark;
use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();
    let (part1, part2) = Benchmark::elapsed("", || run(input));
    print!("Part 1: {part1}\nPart 2: {part2}");
}

fn run(input: &str) -> (usize, usize) {
    let (mut bots, mut queue) = init_bots(input);
    let mut bins = Vec::new();
    let mut part1 = None;

    while queue.len() > 0 {
        let id = queue.pop_front().unwrap();
        let (lower, higher) = bots[&id].get_outputs();

        if lower.value == 17 && higher.value == 61 {
            part1 = Some(id);
        }

        output(lower, &mut bots, &mut queue, &mut bins);
        output(higher, &mut bots, &mut queue, &mut bins);

        if let Some(_) = part1
            && bins.len() == 3
        {
            break;
        }
    }

    (part1.unwrap(), bins.iter().product())
}

fn init_bots(input: &str) -> (HashMap<usize, Bot>, VecDeque<usize>) {
    let mut bindings = Vec::new();
    let mut bots = HashMap::new();

    for line in input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
    {
        if line[0] == "bot" {
            bots.insert(
                as_usize(line[1]),
                Bot::new(
                    OutputType::new(line[5], as_usize(line[6])),
                    OutputType::new(line[10], as_usize(line[11])),
                ),
            );
        } else {
            bindings.push((as_usize(line[1]), as_usize(line[5])));
        }
    }

    let mut queue = VecDeque::new();
    for binding in bindings {
        if bots.get_mut(&binding.1).unwrap().add_chip(binding.0) {
            queue.push_back(binding.1);
        }
    }

    (bots, queue)
}

fn output(
    output: Output,
    bots: &mut HashMap<usize, Bot>,
    queue: &mut VecDeque<usize>,
    bins: &mut Vec<usize>,
) {
    match output.location {
        OutputType::Bot(id) => {
            if bots.get_mut(&id).unwrap().add_chip(output.value) {
                queue.push_front(id);
            }
        }
        OutputType::Bin(id) => {
            if id < 3 {
                bins.push(output.value);
            }
        }
    };
}

fn as_usize(input: &str) -> usize {
    input.parse().unwrap()
}

#[derive(Clone)]
struct Output {
    value: usize,
    location: OutputType,
}

#[derive(Clone)]
enum OutputType {
    Bin(usize),
    Bot(usize),
}

impl OutputType {
    fn new(identifier: &str, value: usize) -> OutputType {
        if identifier == "bot" {
            OutputType::Bot(value)
        } else {
            OutputType::Bin(value)
        }
    }
}

struct Bot {
    chips: [Option<usize>; 2],
    output_locations: (OutputType, OutputType),
}

impl Bot {
    fn new(lower: OutputType, higher: OutputType) -> Bot {
        Bot {
            chips: [None; 2],
            output_locations: (lower, higher),
        }
    }

    //returns true if the robot is holding 2 chips
    fn add_chip(&mut self, chip: usize) -> bool {
        match self.chips[0] {
            None => {
                self.chips[0] = Some(chip);
                false
            }
            Some(_) => {
                self.chips[1] = Some(chip);
                true
            }
        }
    }

    fn get_chips(&self) -> (usize, usize) {
        if self.chips[1] > self.chips[0] {
            (self.chips[0].unwrap(), self.chips[1].unwrap())
        } else {
            (self.chips[1].unwrap(), self.chips[0].unwrap())
        }
    }

    fn get_outputs(&self) -> (Output, Output) {
        let (lower, higher) = self.get_chips();

        (
            Output {
                value: lower,
                location: self.output_locations.0.clone(),
            },
            Output {
                value: higher,
                location: self.output_locations.1.clone(),
            },
        )
    }
}
