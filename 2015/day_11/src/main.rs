use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs, ops::RangeInclusive};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();
    let mut solver = Solver::new();

    let part1 = Benchmark::new(|| solver.get_next(&input));
    println!("Part 1: {}", part1);
    println!(
        "Part 2: {}",
        Benchmark::new(|| solver.get_next(part1.get_output()))
    );
}

struct Solver {
    chars: [char; 23],
    index_map: HashMap<char, usize>,
    straights: Vec<RangeInclusive<usize>>,
    pairs: Vec<RangeInclusive<usize>>,
}

impl Solver {
    fn new() -> Solver {
        let chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't',
            'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut index_map = HashMap::new();
        for (index, c) in chars.iter().enumerate() {
            index_map.insert(*c, index);
        }
        index_map.insert('i', index_map[&'j'] - 1);
        index_map.insert('o', index_map[&'p'] - 1);
        index_map.insert('l', index_map[&'m'] - 1);

        Solver {
            chars,
            index_map,
            straights: Vec::new(),
            pairs: Vec::new(),
        }
    }

    fn get_next(&mut self, input: &str) -> String {
        let mut password: [char; 8] = input.chars().collect::<Vec<_>>().try_into().unwrap();

        loop {
            self.increment_char(&mut password, 7);
            if !(password.contains(&'i') || password.contains(&'o') || password.contains(&'l')) {
                break;
            }
        }

        loop {
            self.straights.clear();
            self.pairs.clear();

            for index in 0..7 {
                self.add_straight(&password.map(|x| x as isize), index);
                self.add_pair(&password, index);
            }

            if self.straights.len() > 0
                && (self.pairs.len() > 3
                    || (self.pairs.len() > 1
                        && self.pairs[0].end() != self.pairs[1].start()
                        && self.pairs[0].start() != self.pairs[1].end()))
            {
                break;
            }

            self.increment_char(&mut password, 7);
        }

        password
            .iter()
            .fold(String::new(), |acc, x| acc + &x.to_string())
    }

    fn increment_char(&mut self, str: &mut [char; 8], index: usize) {
        let i = self.index_map[&str[index]];

        if i == 22 {
            self.increment_char(str, index - 1);
            str[index] = self.chars[0];
        } else {
            str[index] = self.chars[i + 1];
        }
    }

    fn add_straight(&mut self, str: &[isize; 8], index: usize) {
        if index < 6 {
            let a = str[index];
            if str[index + 2] - a == 2 && str[index + 1] - a == 1 {
                self.straights.push((index)..=index + 2);
            }
        }
    }

    fn add_pair(&mut self, str: &[char; 8], index: usize) {
        if index < 7 {
            if str[index] == str[index + 1] {
                self.pairs.push(index..=(index + 1))
            }
        }
    }
}
