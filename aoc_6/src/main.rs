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

fn simulate_cycles(jellyfish_ages: &[u64; 9], cycles: u32) -> u64 {
    let mut ages = jellyfish_ages.clone();
    for _ in 0..cycles {
        ages = (0..ages.len()).rev().fold([0u64; 9], |mut acc, idx| {
            let value = ages[idx];
            if idx == 0 {
                acc[6] += value;
                acc[8] += value;
            } else {
                acc[idx - 1] += value;
            }
            acc
        });
    }
    ages.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input: Vec<u8> = read(path)
        .first()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    let jellyfish_ages = input.iter().fold([0u64; 9], |mut acc, input| {
        acc[*input as usize] += 1;
        acc
    });
    println!("Task 1 result: {}", simulate_cycles(&jellyfish_ages, 80));
    println!("Task 2 result: {}", simulate_cycles(&jellyfish_ages, 256));
}
