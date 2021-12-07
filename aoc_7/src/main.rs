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

fn mean(numbers: &[i32]) -> i32 {
    numbers.iter().sum::<i32>() / numbers.len() as i32
}

fn fuel_consumption(steps: i32) -> i32 {
    (steps.pow(2) + steps) / 2
}

/// Median of a sorted array
///
/// # Arguments
///
/// * `numbers`: sorted list of numbers
///
/// returns: the mean
///
fn median(numbers: &[i32]) -> i32 {
    let len = numbers.len();
    let mid = len / 2 - 1;
    if len % 2 == 0 {
        mean(&numbers[mid - 1..mid + 1])
    } else {
        numbers[len / 2]
    }
}

fn range_of_array(numbers: &[i32]) -> (i32, i32) {
    (*numbers.first().unwrap(), *numbers.last().unwrap())
}

fn calculate_fuel_consumption(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|&num| fuel_consumption((num - target).abs()))
        .sum()
}

fn task_1(positions: &[i32]) -> i32 {
    let middle_position = median(positions);
    positions
        .iter()
        .map( | &num| (num - middle_position).abs()).sum()
}

fn task_2(positions: &[i32]) -> i32 {
    let (start, end) = range_of_array(positions);
    // this range could be reduced to mean +- 1/2
    // according to https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
    // so checking mean and its neighbors is faster but my solution is also fast enough
    (start..end+1)
        .map(|value| calculate_fuel_consumption(positions, value))
        .min()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut input: Vec<i32> = read(path)
        .first()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    input.sort();
    println!("Task 1 result: {}", task_1(&input));
    println!("Task 2 result: {}", task_2(&input));
}
