use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Cannot open file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Cannot parse string").trim().to_string())
        .collect()
}

fn get_easy_number(input: &BTreeSet<char>) -> Option<u8> {
    match input.len() {
        2 => Option::Some(1),
        3 => Option::Some(7),
        4 => Option::Some(4),
        7 => Option::Some(8),
        _ => Option::None,
    }
}

fn task_2(lookup_map: &Vec<(u8, Vec<(u8, u8)>)>, observation: &Observation) -> u32 {
    let mut number_representations =
        observation
            .signals
            .iter()
            .fold(HashMap::new(), |mut acc, set| {
                match get_easy_number(set) {
                    Option::Some(val) => {
                        acc.insert(val, set.clone());
                    }
                    Option::None => {}
                };
                acc
            });
    for value in observation.signals.iter() {
        if !number_representations.values().any(|val| val == value) {
            let number = lookup_map
                .iter()
                .find(|(_, val)| {
                    val.iter()
                        .all(|(n_key, n_val)| match number_representations.get(n_key) {
                            Option::Some(x) => value.intersection(x).count() == *n_val as usize,
                            Option::None => false,
                        })
                })
                .unwrap()
                .0;
            number_representations.insert(number, value.clone());
        }
    }
    observation
        .result
        .iter()
        .map(|num| {
            number_representations
                .iter()
                .find(|(_, val)| num == *val)
                .unwrap()
                .0
        })
        .rev()
        .enumerate()
        .fold(0u32, |acc, (idx, &val)| {
            acc + val as u32 * 10u32.pow(idx as u32)
        })
}

#[derive(Debug)]
struct Observation {
    signals: Vec<BTreeSet<char>>,
    result: Vec<BTreeSet<char>>,
}

fn task_1(input: &Vec<Observation>) -> u32 {
    input
        .iter()
        .map(|observation| {
            observation
                .result
                .iter()
                .map(|set| match get_easy_number(set) {
                    Option::Some(_) => 1u32,
                    Option::None => 0u32,
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let lookup_map: Vec<(u8, Vec<(u8, u8)>)> = vec![
        (0, vec![(1, 2), (4, 3), (7, 3), (8, 6)]),
        (2, vec![(1, 1), (4, 2), (7, 2), (8, 5)]),
        (3, vec![(1, 2), (4, 3), (7, 3), (8, 5)]),
        (5, vec![(1, 1), (4, 3), (7, 2), (8, 5)]),
        (6, vec![(1, 1), (4, 3), (7, 2), (8, 6)]),
        (9, vec![(1, 2), (4, 4), (7, 3), (8, 6)]),
    ];
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input: Vec<Observation> = read(path)
        .into_iter()
        .map(|line| {
            let mut parts: Vec<Vec<BTreeSet<char>>> = line
                .split(" | ")
                .map(|part| {
                    part.split_whitespace()
                        .map(|value| BTreeSet::from_iter(value.chars()))
                        .collect()
                })
                .collect();
            let result = parts.pop().unwrap();
            let signals = parts.pop().unwrap();
            Observation { signals, result }
        })
        .collect();

    println!("Task 1 result: {}", task_1(&input));
    let task_2 = input
        .iter()
        .map(|val| task_2(&lookup_map, val))
        .sum::<u32>();
    println!("Task 2 result: {}", task_2)
}
