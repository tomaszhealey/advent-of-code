use primitive_benchmarker::Benchmark;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let input = fs::read_to_string(file_path).expect("Error reading file.");
    let input = input.trim_end();

    let ((rooms, part1), elapsed) = Benchmark::new(|| part1(&input)).to_tuple();
    println!("Part 1: {part1}, Elapsed: {elapsed:2?}");

    println!("Part 2: {}", Benchmark::new(|| part2(rooms)));
}

struct Room<'a> {
    id: usize,
    name: Vec<&'a str>,
    is_real: bool,
}

impl Room<'_> {
    fn new<'a>(s: &'a str) -> Room<'a> {
        let mut parts: Vec<&str> = s.split("-").collect();
        let end: Vec<&str> = parts.pop().unwrap().split("[").collect();
        let mut count = HashMap::new();
        let name: Vec<&str> = parts.clone();

        for segment in parts {
            for c in segment.chars() {
                let entry = count.entry(c).or_insert(0);
                *entry += 1;
            }
        }

        let mut count: Vec<_> = count.iter().collect();
        sort(&mut count);

        Room {
            id: end[0].parse().unwrap(),
            name,
            is_real: count[0..5].iter().map(|p| p.0).collect::<String>()
                == end[1].strip_suffix("]").unwrap(),
        }
    }
}

fn sort(list: &mut Vec<(&char, &usize)>) {
    for i in 1..list.len() {
        let value = list[i].clone();
        let mut index = 0;
        for j in (0..i).rev() {
            if value.1 > list[j].1 || (value.1 == list[j].1 && value.0 < list[j].0) {
                list[j + 1] = list[j].clone();
            } else {
                index = j + 1;
                break;
            }
        }
        list[index] = value;
    }
}

fn part1<'a>(input: &'a str) -> (Vec<Room<'a>>, usize) {
    let lines = input
        .lines()
        .map(|l| Room::new(l))
        .filter(|room| room.is_real);

    (
        lines.clone().collect::<Vec<_>>(),
        lines.fold(0, |acc, room| acc + room.id),
    )
}

fn part2(rooms: Vec<Room>) -> usize {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    for room in rooms {
        let mut name = String::new();
        for word in room.name {
            for c in word.chars() {
                name.push(letters[(letters.iter().position(|&x| x == c).unwrap() + room.id) % 26]);
            }
            name.push(' ');
        }
        if name == "northpole object storage " {
            return room.id;
        }
    }

    0
}
