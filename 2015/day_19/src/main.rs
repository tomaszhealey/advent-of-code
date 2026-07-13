use primitive_benchmarker::Benchmark;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let solver = Solver::new(input.trim_end());

    println!("Part 1: {}", Benchmark::new(|| solver.part1()));
    println!("Part 2: {}", Benchmark::new(|| solver.part2()));
}

struct Solver<'a> {
    mol: &'a str,
    replacements: HashMap<&'a str, Vec<&'a str>>,
}

impl Solver<'_> {
    fn new(input: &str) -> Solver<'_> {
        let mut replacements = HashMap::new();

        let mut lines = input.lines().rev();
        let mol = lines.next().unwrap();
        for mut line in lines.skip(1).map(|x| x.split(" => ")) {
            let entry = replacements
                .entry(line.next().unwrap())
                .or_insert(Vec::new());
            entry.push(line.next().unwrap());
        }

        Solver { mol, replacements }
    }

    fn part1(&self) -> usize {
        let mut possible = HashSet::new();
        self.check_replacements(&mut possible, 1);
        self.check_replacements(&mut possible, 2);
        possible.len()
    }

    fn check_replacements(&self, possible: &mut HashSet<String>, window_size: usize) {
        for i in 0..self.mol.len() - (window_size - 1) {
            let s = &self.mol[i..i + window_size];
            if let Some(strs) = self.replacements.get(s) {
                let pre = &self.mol[..i];
                let post = &self.mol[i + window_size..self.mol.len()];
                for replacement in strs {
                    possible.insert(String::from(pre) + replacement + post);
                }
            }
        }
    }

    //credit to u/askalski for this solution https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
    fn part2(&self) -> usize {
        let chars: Vec<char> = self.mol.chars().collect();
        let mut count = 0;
        for i in 0..self.mol.len() {
            if chars[i].is_ascii_lowercase() {
                continue;
            }

            if chars[i] == 'Y' {
                count -= 1;
            } else if chars[i] != 'R' && !(chars[i] == 'A' && self.mol[i..i + 2] == *"Ar") {
                count += 1;
            }
        }

        count - 1
    }
}
