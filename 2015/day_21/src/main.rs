use benchmarker::benchmark;
use std::{
    collections::{HashMap, VecDeque},
    env, fs, usize,
};

mod shop;
use shop::{Item, Shop};

fn main() {
    let boss = Boss::new();
    let shop = Shop::new();

    //tts: Time to Survive
    println!(
        "Part 1: {}",
        benchmark(|| find_gold(
            usize::MAX,
            &boss,
            &shop,
            |stored, new| stored > new,
            |tts: f64| (boss.damage as f64 - (100f64 / tts)) as usize + 1,
        ))
    );
    println!(
        "Part 2: {}",
        benchmark(|| find_gold(
            0,
            &boss,
            &shop,
            |stored, new| stored < new,
            |tts: f64| (boss.damage as f64 - (100f64 / tts)) as usize,
        ))
    );
}

struct Boss {
    hp: usize,
    damage: usize,
    armour: usize,
}

impl Boss {
    fn new() -> Boss {
        let file_path = env::args()
            .nth(1)
            .unwrap_or_else(|| String::from("input.txt"));
        let input = fs::read_to_string(file_path).expect("Error reading file.");
        let mut input = input.trim_end().lines().map(|x| {
            x.split(" ")
                .collect::<VecDeque<_>>()
                .pop_back()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        });

        Boss {
            hp: input.next().unwrap(),
            damage: input.next().unwrap(),
            armour: input.next().unwrap(),
        }
    }
}

fn find_best(
    items: &Vec<Item>,
    rings: &Vec<Item>,
    is_better: &impl Fn(usize, usize) -> bool,
) -> HashMap<usize, usize> {
    let mut stats = HashMap::new();
    for item in items {
        for ring in rings {
            let stat = item.stat + ring.stat;
            let cost = item.cost + ring.cost;
            if !stats.contains_key(&stat) || is_better(stats[&stat], cost) {
                stats.insert(stat, cost);
            }
        }
    }

    stats
}

fn find_gold(
    mut acc: usize,
    boss: &Boss,
    shop: &Shop,
    is_better: impl Fn(usize, usize) -> bool,
    get_def: impl Fn(f64) -> usize,
) -> usize {
    let damages = find_best(&shop.weapons, &shop.damage_rings, &is_better);
    let defenses = find_best(&shop.armour, &shop.armour_rings, &is_better);
    for (damage, cost) in damages {
        let tts = boss.hp / (damage - boss.armour);
        let def = get_def(tts as f64);
        if defenses.contains_key(&def) {
            let def_cost = defenses[&def];
            if is_better(acc, cost + def_cost) {
                acc = cost + def_cost;
            }
        }
    }
    acc
}
