use benchmarker::benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let mut solver = Solver::new(input.trim_end());

    println!("Part 1: {}", benchmark(|| solver.part1()));
    println!("Part 2: {}", benchmark(|| solver.part2()));
}

struct Solver<'a> {
    graph: HashMap<&'a str, HashMap<&'a str, isize>>,
}

impl Solver<'_> {
    fn new(input: &str) -> Solver<'_> {
        let mut output = HashMap::new();
        let mut mult: isize;

        for line in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
            mult = if line[2] == "gain" { 1 } else { -1 };
            let person = output.entry(line[0]).or_insert(HashMap::new());
            person.insert(
                line[10].strip_suffix(".").unwrap(),
                mult * line[3].parse::<isize>().unwrap(),
            );
        }

        Solver { graph: output }
    }

    fn part1(&self) -> isize {
        self.find_best(
            &mut vec![""; self.graph.keys().len()],
            self.graph.keys().map(|x| *x).collect(),
        )
    }

    fn part2(&mut self) -> isize {
        self.graph.insert("me", HashMap::new());
        for key in self.graph.clone().keys() {
            self.graph.get_mut(key).unwrap().insert("me", 0);
            self.graph.get_mut("me").unwrap().insert(key, 0);
        }

        self.part1()
    }

    fn find_best<'a>(&self, table: &mut Vec<&'a str>, stack: Vec<&'a str>) -> isize {
        if stack.len() == 0 {
            let len = table.len();
            let mut acc = self.graph[table[0]][table[len - 1]] + self.graph[table[0]][table[1]];
            for i in 1..table.len() {
                acc +=
                    self.graph[table[i]][table[i - 1]] + self.graph[table[i]][table[(i + 1) % len]];
            }
            return acc;
        }

        let mut best = isize::MIN;
        for next in 0..stack.len() {
            let new: Vec<&str> = stack
                .clone()
                .iter()
                .filter(|x| **x != stack[next])
                .map(|x| *x)
                .collect();
            table[self.graph.len() - stack.len()] = stack[next];

            let out = self.find_best(table, new);
            if out > best {
                best = out;
            }
        }

        best
    }
}
