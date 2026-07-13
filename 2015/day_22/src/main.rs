use primitive_benchmarker::Benchmark;
use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};
use Spell::{Drain, MagicMissile, Poison, Recharge, Shield};

fn main() {
    let (boss_health, boss_damage) = get_boss();

    println!(
        "Part 1: {}",
        Benchmark::new(|| run(boss_health, boss_damage, false))
    );
    println!(
        "Part 2: {}",
        Benchmark::new(|| run(boss_health, boss_damage, true))
    );
}

fn get_boss() -> (isize, isize) {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let mut input = input.trim_end().lines().map(|l| {
        l.split(": ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<isize>()
            .unwrap()
    });

    (
        input.next().unwrap(),
        input.next().unwrap(), //damage dealt by boss
    )
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
struct Instance {
    hp: isize,
    mana: usize,
    timers: HashMap<Spell, isize>,
    boss_hp: isize,
    mana_used: usize,
}

impl Instance {
    fn new(boss_hp: isize) -> Instance {
        let timers = HashMap::from([(Shield, 0), (Poison, 0), (Recharge, 0)]);
        Instance {
            hp: 50,
            mana: 500,
            mana_used: 0,
            timers,
            boss_hp,
        }
    }

    fn mutate(&self, spell_cost: usize, player_damage: isize, spell: &Spell) -> Instance {
        let mut instance = self.clone();
        instance.boss_hp -= player_damage;

        if instance.timers.contains_key(spell) {
            match spell {
                Poison | Shield => *instance.timers.get_mut(spell).unwrap() = 6,
                Recharge => *instance.timers.get_mut(spell).unwrap() = 5,
                _ => {}
            };
        }

        if let Drain = spell {
            instance.hp += 2;
        }

        instance.mana -= spell_cost;
        instance.mana_used += spell_cost;

        instance
    }

    fn damage(&mut self, damage: isize) {
        self.hp -= damage as isize;
        if self.timers[&Shield] > 0 {
            self.hp += 7
        }
    }

    fn update_timers(&mut self) {
        for (spell, timer) in self.timers.iter_mut() {
            if *timer > 0 {
                match spell {
                    Recharge => self.mana += 101,
                    Poison => self.boss_hp -= 3,
                    _ => {}
                }
            }
            *timer -= 1;
        }
    }
}

fn run(boss_hp: isize, boss_damage: isize, is_part2: bool) -> usize {
    let mut best = usize::MAX;
    let mut queue = VecDeque::new();
    let spells = vec![
        (53, 4, MagicMissile),
        (73, 2, Drain),
        (113, 0, Shield),
        (173, 0, Poison),
        (229, 0, Recharge),
    ];
    queue.push_back(Instance::new(boss_hp));

    while queue.len() > 0 {
        let mut instance = queue.pop_front().unwrap();

        if is_part2 {
            instance.hp -= 1;
        }

        if instance.boss_hp <= 0 && instance.mana_used < best {
            best = instance.mana_used;
        } else if instance.mana_used > best || instance.mana <= 53 || instance.hp <= 0 {
            continue;
        }

        //Player's Turn
        instance.update_timers();
        for (cost, damage, spell) in &spells {
            let timer = *instance.timers.get(&spell).unwrap_or_else(|| &-1);
            if *cost <= instance.mana && timer < 2 {
                let mut n_instance = instance.mutate(*cost, *damage, spell);

                //Boss's Turn
                n_instance.update_timers();
                n_instance.damage(boss_damage);

                queue.push_back(n_instance);
            }
        }
    }
    best
}
