use primitive_benchmarker::Benchmark;
use std::{env, fs, vec};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(
            &input,
            (1, 1),
            vec![
                vec!["1", "2", "3"],
                vec!["4", "5", "6"],
                vec!["7", "8", "9"]
            ]
        ))
    );

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(
            &input,
            (2, 0),
            vec![
                vec!["", "", "1", "", ""],
                vec!["", "2", "3", "4", ""],
                vec!["5", "6", "7", "8", "9"],
                vec!["", "A", "B", "C", ""],
                vec!["", "", "D", "", ""]
            ]
        ))
    );
}

struct ValidDirections {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

struct Position<'a> {
    row: usize,
    col: usize,
    numpad: Vec<Vec<&'a str>>,
    valid_dirs: Vec<Vec<ValidDirections>>,
}

impl Position<'_> {
    fn new<'a>(row: usize, col: usize, numpad: Vec<Vec<&'a str>>) -> Position<'a> {
        let mut valid_dirs = Vec::new();
        let len = numpad.len(); //numpads are square
        for row in 0..len {
            let mut valid = Vec::new();
            for col in 0..len {
                valid.push(ValidDirections {
                    up: row > 0 && numpad[row - 1][col] != "",
                    down: row < len - 1 && numpad[row + 1][col] != "",
                    left: col > 0 && numpad[row][col - 1] != "",
                    right: col < len - 1 && numpad[row][col + 1] != "",
                })
            }
            valid_dirs.push(valid);
        }

        Position {
            row,
            col,
            numpad,
            valid_dirs,
        }
    }

    fn update(&mut self, dir: char) {
        let valid = &self.valid_dirs[self.row][self.col];
        match dir {
            'U' if valid.up => self.row -= 1,
            'R' if valid.right => self.col += 1,
            'L' if valid.left => self.col -= 1,
            'D' if valid.down => self.row += 1,
            _ => {}
        }
    }

    fn get_key(&self) -> &str {
        self.numpad[self.row][self.col]
    }
}

fn run(input: &str, start: (usize, usize), numpad: Vec<Vec<&str>>) -> String {
    let mut code = String::new();
    let mut position = Position::new(start.0, start.1, numpad);

    for line in input.lines() {
        for c in line.chars() {
            position.update(c);
        }
        code.push_str(position.get_key());
    }

    code
}
