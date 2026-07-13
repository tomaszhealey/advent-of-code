use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let mut solver = Solver::new(&input.trim_end());

    println!("Part 1: {}", Benchmark::new(|| solver.part1()));
    println!("Part 2: {}", Benchmark::new(|| solver.part2()));
}

enum SolverMode {
    Part1,
    Part2,
}

struct Solver<'a> {
    graph: HashMap<&'a str, HashMap<&'a str, usize>>,
    max_len: usize,
    solver_mode: SolverMode,
}

impl Solver<'_> {
    fn new(input: &str) -> Solver<'_> {
        let mut graph = HashMap::new();

        for edge in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
            let cost = edge[4].parse::<usize>().unwrap();
            Solver::add_node(edge[0], edge[2], cost, &mut graph);
            Solver::add_node(edge[2], edge[0], cost, &mut graph);
        }

        Solver {
            max_len: graph.len(),
            graph,
            solver_mode: SolverMode::Part1,
        }
    }

    fn add_node<'a>(
        n1: &'a str,
        n2: &'a str,
        cost: usize,
        graph: &mut HashMap<&'a str, HashMap<&'a str, usize>>,
    ) {
        let node = graph.entry(n1).or_insert(HashMap::new());
        node.insert(n2, cost);
    }

    fn find_path(&self, next: &str, dist: usize, best: usize, path: Vec<&str>) -> Option<usize> {
        match self.solver_mode {
            SolverMode::Part1 => {
                if dist >= best {
                    return None;
                } else if path.len() == self.max_len {
                    return Some(dist);
                }
            }
            SolverMode::Part2 => {
                if path.len() == self.max_len {
                    if dist >= best {
                        return Some(dist);
                    }
                    return None;
                }
            }
        }

        Some(self.traverse_all(next, dist, best, path))
    }

    fn traverse_all(&self, next: &str, dist: usize, mut best: usize, path: Vec<&str>) -> usize {
        for (node, cost) in self.graph[next].clone() {
            if path.contains(&node) {
                continue;
            }

            let mut npath = path.clone();
            npath.push(node);

            if let Some(val) = self.find_path(node, dist + cost, best, npath) {
                best = val;
            }
        }

        best
    }

    fn part1(&mut self) -> usize {
        self.solver_mode = SolverMode::Part1;

        let mut best = usize::MAX;
        for node in self.graph.keys() {
            best = self.traverse_all(node, 0, best, vec![node]);
        }

        best
    }

    fn part2(&mut self) -> usize {
        self.solver_mode = SolverMode::Part2;

        let mut best = 0;
        for node in self.graph.keys() {
            best = self.traverse_all(node, 0, best, vec![node]);
        }

        best
    }
}
