use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = Benchmark::elapsed("Parsing", || parse(&input));

    let part1 = Benchmark::new(|| get_reg("a", &input, &mut HashMap::new()));
    println!("Part 1: {}", part1);

    let mut regs = HashMap::new();
    regs.insert("b", *part1.get_output());
    println!(
        "Part 2: {}",
        Benchmark::new(|| get_reg("a", &input, &mut regs))
    );
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut instrs = HashMap::new();

    for mut instr in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
        instrs.insert(instr.pop().unwrap(), instr.split_last().unwrap().1.to_vec());
    }

    instrs
}

fn get_val<'a>(
    input: &'a str,
    instrs: &'a HashMap<&str, Vec<&str>>,
    regs: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(val) = regs.get(input) {
        return *val;
    } else if let Ok(val) = input.parse::<u16>() {
        regs.insert(input, val);
        return val;
    }
    let val = get_reg(input, instrs, regs);
    regs.insert(input, val);
    val
}

fn get_reg<'a>(
    reg: &'a str,
    instrs: &'a HashMap<&str, Vec<&str>>,
    regs: &mut HashMap<&'a str, u16>,
) -> u16 {
    let instr = &instrs.get(reg).expect(&format!("{reg}"));
    match instr.len() {
        1 => return get_val(instr[0], instrs, regs),
        2 => return !get_val(instr[1], instrs, regs),
        _ => match instr[1] {
            "AND" => return get_val(instr[0], instrs, regs) & get_val(instr[2], instrs, regs),
            "OR" => return get_val(instr[0], instrs, regs) | get_val(instr[2], instrs, regs),
            "LSHIFT" => return get_val(instr[0], instrs, regs) << instr[2].parse::<u16>().unwrap(),
            "RSHIFT" => return get_val(instr[0], instrs, regs) >> instr[2].parse::<u16>().unwrap(),
            _ => panic!("Unknown instr."),
        },
    }
}
