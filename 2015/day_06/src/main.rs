use primitive_benchmarker::Benchmark;
use std::{env, fs};
use InstructionType::{Off, On, Toggle};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = parse_instructions(input.trim_end());

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(&input, |_| 1, |_| 0, |x| x ^ 1,))
    );

    println!(
        "Part 2: {}",
        Benchmark::new(|| run(
            &input,
            |x| x + 1,
            |x| if x > 0 { x - 1 } else { 0 },
            |x| x + 2
        ))
    )
}

enum InstructionType {
    Toggle,
    On,
    Off,
}

struct Vector2 {
    x: usize,
    y: usize,
}

impl Vector2 {
    fn new(input: &str) -> Vector2 {
        let mut input = input.split(",").map(|x| x.parse::<usize>().unwrap());
        Vector2 {
            x: input.next().unwrap(),
            y: input.next().unwrap(),
        }
    }
}

struct Instruction {
    instr_type: InstructionType,
    start: Vector2,
    end: Vector2,
}

impl Instruction {
    fn new(instr_type: InstructionType, str1: &str, str2: &str) -> Instruction {
        Instruction {
            instr_type,
            start: Vector2::new(str1),
            end: Vector2::new(str2),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instrs: Vec<Instruction> = Vec::new();

    for instr in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
        if instr.len() == 5 {
            match instr[1] {
                "on" => instrs.push(Instruction::new(On, instr[2], instr[4])),
                "off" => instrs.push(Instruction::new(Off, instr[2], instr[4])),
                _ => panic!("Failed to parse instructions"),
            };
        } else {
            instrs.push(Instruction::new(Toggle, instr[1], instr[3]));
        }
    }

    instrs
}

fn run(
    instrs: &Vec<Instruction>,
    on: impl Fn(u8) -> u8,
    off: impl Fn(u8) -> u8,
    toggle: impl Fn(u8) -> u8,
) -> usize {
    //Luckily, no light's value exceeds u8::MAX
    let mut lights = Box::from([[0u8; 1000]; 1000]);

    for instr in instrs {
        match instr.instr_type {
            On => update_lights(&mut lights, &instr.start, &instr.end, &on),
            Off => update_lights(&mut lights, &instr.start, &instr.end, &off),
            Toggle => update_lights(&mut lights, &instr.start, &instr.end, &toggle),
        }
    }

    lights.iter().fold(0usize, |acc, x| {
        acc + x.iter().fold(0usize, |a1, y| a1 + *y as usize)
    })
}

fn update_lights(
    lights: &mut [[u8; 1000]],
    start: &Vector2,
    end: &Vector2,
    update: &impl Fn(u8) -> u8,
) {
    for row in start.y..end.y + 1 {
        for col in start.x..end.x + 1 {
            lights[row][col] = update(lights[row][col]);
        }
    }
}
