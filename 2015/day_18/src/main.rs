use benchmarker::benchmark;
use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let lights = parse(input.trim_end());

    println!("Part 1: {}", benchmark(|| run(lights.clone(), 100, false)));

    let mut part2 = lights.clone();
    let len = part2.len();
    part2[1][1] = true;
    part2[1][len - 2] = true;
    part2[len - 2][1] = true;
    part2[len - 2][len - 2] = true;
    println!("Part 2: {}", benchmark(|| run(part2, 100, true)));
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    //all sides of lights are padded with off lights so that I don't have to check boundaries
    //when checking surrounding lights.
    let mut lights = Vec::new();
    let off_row = vec![false; input.lines().next().unwrap().len() + 2];
    lights.push(off_row.clone());

    for line in input.lines() {
        let mut row = vec![false];
        for light in line.chars() {
            row.push(light == '#');
        }
        row.push(false);
        lights.push(row)
    }

    lights.push(off_row);

    lights
}

fn run(mut lights: Vec<Vec<bool>>, steps: usize, is_part2: bool) -> usize {
    let len = lights.len(); //board is square
    for _ in 0..steps {
        let mut new_lights = lights.clone();
        for row in 1..len - 1 {
            for col in 1..len - 1 {
                let surrounding = get_surroundings(&lights, row, col);
                if lights[row][col] && surrounding != 2 && surrounding != 3 {
                    if !(is_part2 && is_corner(row, col, len)) {
                        new_lights[row][col] = false;
                    }
                } else if !lights[row][col] && (surrounding == 3) {
                    new_lights[row][col] = true;
                }
            }
        }
        lights = new_lights;
    }

    lights
        .iter()
        .fold(0, |acc, line| acc + line.iter().filter(|l| **l).count())
}

fn get_surroundings(lights: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut on = 0;
    for r in (row - 1)..=(row + 1) {
        for c in (col - 1)..=(col + 1) {
            if r == row && c == col {
                continue;
            } else if lights[r][c] == true {
                on += 1;
            }
        }
    }
    on
}

fn is_corner(row: usize, col: usize, line_len: usize) -> bool {
    (row == 1 || row == line_len - 2) && (col == 1 || col == line_len - 2)
}
