use benchmarker::benchmark;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let mut solver = Solver::new(input.trim_end());

    println!("Part 1: {}", benchmark(|| solver.part1()));
    println!("Part 2: {}", benchmark(|| solver.part2()));
}

fn as_isize(input: &str) -> isize {
    input.strip_suffix(",").unwrap_or(input).parse().unwrap()
}

#[derive(PartialEq, Eq, Clone)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavour: isize,
    texture: isize,
    calories: isize,
}

impl Ingredient {
    fn blank() -> Self {
        Self {
            capacity: 0,
            durability: 0,
            flavour: 0,
            texture: 0,
            calories: 0,
        }
    }

    fn add(self, lhs: &Self) -> Self {
        Self {
            capacity: self.capacity + lhs.capacity,
            durability: self.durability + lhs.durability,
            flavour: self.flavour + lhs.flavour,
            texture: self.texture + lhs.texture,
            calories: self.calories + lhs.calories,
        }
    }

    fn breakdown(&self) -> [isize; 5] {
        [
            self.capacity,
            self.durability,
            self.flavour,
            self.texture,
            self.calories,
        ]
    }
}

struct Solver {
    ingredients: Vec<Ingredient>,
    cache: HashSet<Vec<usize>>,
    is_part2: bool,
}

impl Solver {
    fn new(input: &str) -> Solver {
        let mut ingredients = Vec::new();
        for line in input.lines().map(|x| x.split(" ").collect::<Vec<_>>()) {
            ingredients.push(Ingredient {
                capacity: as_isize(line[2]),
                durability: as_isize(line[4]),
                flavour: as_isize(line[6]),
                texture: as_isize(line[8]),
                calories: as_isize(line[10]),
            });
        }

        Solver {
            ingredients,
            cache: HashSet::new(),
            is_part2: false,
        }
    }

    fn part1(&mut self) -> isize {
        self.is_part2 = false;
        self.cache.clear();
        self.add_spoon(Vec::new())
    }

    fn part2(&mut self) -> isize {
        self.is_part2 = true;
        self.cache.clear();
        self.add_spoon(Vec::new())
    }

    fn add_spoon<'a>(&mut self, spoons: Vec<&'a Ingredient>) -> isize {
        if self.is_part2 && spoons.iter().fold(0, |acc, x| acc + x.calories) > 500 {
            return 0;
        } else if spoons.len() == 100 {
            self.cache.insert(self.count_ingredients(&spoons));

            let total = spoons
                .iter()
                .fold(Ingredient::blank(), |acc, x| acc.add(x))
                .breakdown();

            if self.is_part2 && total[4] != 500 {
                return 0;
            }

            let total = &total[..4];
            if total.iter().any(|x| *x <= 0) {
                return 0;
            } else {
                return total.iter().product();
            }
        }

        let mut best = 0;
        for ingredient in self.ingredients.clone().iter() {
            let mut new = spoons.clone();
            new.push(ingredient);

            let count = self.count_ingredients(&spoons);
            if !self.cache.contains(&count) {
                let out = self.add_spoon(new);
                if out > best {
                    best = out;
                }
            }
        }

        self.cache.insert(self.count_ingredients(&spoons));

        best
    }

    fn count_ingredients(&self, spoons: &Vec<&Ingredient>) -> Vec<usize> {
        let mut count = Vec::new();

        for ingredient in self.ingredients.iter() {
            count.push(
                spoons
                    .iter()
                    .fold(0, |acc, x| if *x == ingredient { acc + 1 } else { acc }),
            );
        }

        count
    }
}
