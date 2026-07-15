use primitive_benchmarker::Benchmark;
use std::{env, fs};
use Instruction::{RCol, RRow, Rect};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let instrs = parse(&input);

    let (lights, screen) = Benchmark::elapsed("", || run(instrs));
    println!("Part 1: {lights}\nPart 2:\n{screen}");
}

fn as_usize(input: &str) -> usize {
    input.parse().unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    input.trim_end().lines().map(|l| {
        let line = l.split_whitespace().collect::<Vec<_>>();
        match line[1] {
            "row" => RRow(
                as_usize(line[2].strip_prefix("y=").unwrap()),
                as_usize(line[4]),
            ),
            "column" => RCol(
                as_usize(line[2].strip_prefix("x=").unwrap()),
                as_usize(line[4]),
            ),
            _ => {
                let mut dims = line[1].split("x").map(|n| as_usize(n));
                Rect(dims.next().unwrap(), dims.next().unwrap())
            }
        }
    })
}

enum Instruction {
    Rect(usize, usize),
    RRow(usize, usize),
    RCol(usize, usize),
}

struct Dimensions {
    rows: usize,
    cols: usize,
}

fn run(instrs: impl Iterator<Item = Instruction>) -> (usize, String) {
    const DIMS: Dimensions = Dimensions { rows: 6, cols: 50 };
    let mut lcd = [['.'; DIMS.cols]; DIMS.rows];

    for instr in instrs {
        match instr {
            Rect(cols, rows) => {
                for r in 0..rows {
                    for c in 0..cols {
                        lcd[r][c] = '#';
                    }
                }
            }
            RRow(row, i) => {
                for (index, pixel) in lcd[row].clone().iter().enumerate() {
                    lcd[row][(index + i) % DIMS.cols] = *pixel;
                }
            }
            RCol(col, i) => {
                for (index, pixel) in lcd.clone().map(|row| row[col]).iter().enumerate() {
                    lcd[(index + i) % DIMS.rows][col] = *pixel;
                }
            }
        }
    }

    //Count on lights
    let lights = lcd.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&pixel| pixel == '#').count()
    });

    //Format LCD into a String
    let mut output = String::new();
    for row in lcd {
        for pixel in row {
            output.push(pixel);
        }
        output.push('\n');
    }

    (lights, output)
}
