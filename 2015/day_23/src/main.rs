use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs};
use Instruction::{Hlf, Inc, Jie, Jio, Jmp, Tpl};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let instructions = parse(input.trim_end());

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(&instructions, HashMap::from([("a", 0), ("b", 0)])))
    );
    println!(
        "Part 2: {}",
        Benchmark::new(|| run(&instructions, HashMap::from([("a", 1), ("b", 0)])))
    );
}

enum Instruction<'a> {
    Hlf(&'a str),
    Tpl(&'a str),
    Inc(&'a str),
    Jmp(isize),
    Jie(&'a str, isize),
    Jio(&'a str, isize),
}

fn parse<'a>(input: &'a str) -> Vec<Instruction<'a>> {
    input
        .lines()
        .map(|l| {
            let v = l.split(" ").collect::<Vec<_>>();
            match v[0] {
                "hlf" => Hlf(v[1]),
                "tpl" => Tpl(v[1]),
                "inc" => Inc(v[1]),
                "jmp" => Jmp(v[1].parse().unwrap()),
                "jie" => Jie(v[1].strip_suffix(",").unwrap(), v[2].parse().unwrap()),
                "jio" => Jio(v[1].strip_suffix(",").unwrap(), v[2].parse().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect::<Vec<_>>()
}
fn run(instructions: &Vec<Instruction>, mut regs: HashMap<&str, usize>) -> usize {
    let mut pntr = 0;
    while pntr < instructions.len() {
        pntr = match instructions[pntr] {
            Hlf(reg) => update(reg, pntr, |reg| *regs.get_mut(reg).unwrap() /= 2),
            Tpl(reg) => update(reg, pntr, |reg| *regs.get_mut(reg).unwrap() *= 3),
            Inc(reg) => update(reg, pntr, |reg| *regs.get_mut(reg).unwrap() += 1),
            Jmp(offset) => jump("", pntr, offset, |_| true),
            Jie(reg, offset) => jump(reg, pntr, offset, |reg| regs[reg] % 2 == 0),
            Jio(reg, offset) => jump(reg, pntr, offset, |reg| regs[reg] == 1),
        };
    }

    regs["b"]
}

fn update(reg: &str, pntr: usize, mut operation: impl FnMut(&str)) -> usize {
    operation(reg);
    pntr + 1
}

fn jump(reg: &str, pntr: usize, offset: isize, mut condition: impl FnMut(&str) -> bool) -> usize {
    if condition(reg) {
        (pntr as isize + offset) as usize
    } else {
        pntr + 1
    }
}
