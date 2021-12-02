use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

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

fn calculate_number_increases(numbers: &[u32]) -> u32 {
    return numbers.windows(2).fold(0, |acc, value|
        if value[0] < value[1] {
            acc + 1
        } else {
            acc
        });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let numbers: Vec<u32> = read(path).into_iter().map(|line| line.parse().expect("Expecting integer")).collect();
    let increased_numbers: u32 = calculate_number_increases(&numbers[..]);
    println!("Part 1 result: {}", increased_numbers);
    let three_window_sums: Vec<u32> = numbers[..].windows(3).map(| window | window.iter().sum()).collect();
    let increased_three_window_sums = calculate_number_increases(&three_window_sums[..]);
    println!("Part 2 result: {}", increased_three_window_sums);
}
