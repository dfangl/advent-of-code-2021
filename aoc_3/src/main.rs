use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

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

fn sum_position(numbers: &Vec<Vec<u32>>, pos: usize) -> u32 {
    return numbers.iter().map(| line | line[pos]).sum();
}

fn most_common(numbers: &Vec<Vec<u32>>, pos: usize) -> u32 {
    let ones_count = sum_position(numbers, pos);
    let zeroes_count = numbers.len() as u32 - ones_count;
    //return match ones_count.partial_cmp(&zeroes_count).unwrap() {
    //        Ordering::Less => 0,
    //        Ordering::Equal | Ordering::Greater => 1,
    //    }

    return if ones_count >= zeroes_count { 1 } else { 0 };
}


fn filter_candidates(numbers: &Vec<Vec<u32>>, common_function: fn(&Vec<Vec<u32>>, pos: usize) -> u32) -> u32 {
    let mut pos = 0;
    let mut numbers = numbers.clone();
    while numbers.len() > 1 {
        let least_common = common_function(&numbers, pos);
        numbers = numbers.into_iter().filter(|candidate | {
            candidate[pos] == least_common
        }).collect();
        pos += 1;
    }
    let number = numbers.first().unwrap().iter().map(|num | num.to_string()).collect::<String>();
    return u32::from_str_radix(number.as_str(), 2).unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let report: Vec<String> = read(path);

    let report_item_len = report[0].len();
    let numbers: Vec<Vec<u32>> = report.iter().map(| line |
        line.chars().map(|d| d.to_digit(2).unwrap()).collect()
    ).collect();
    let one_counts: Vec<u32> = numbers.iter().fold(vec![0; report_item_len], |mut acc, values | {
        for (pos, value) in values.iter().enumerate() {
            acc[pos] += *value
        }
        acc
    });
    let report_len = numbers.len() as u32;

    // create solution string
    // FIXME this only works on even number of reports
    let final_number: String = one_counts.iter().map(|number | {
        match number.partial_cmp(&(report_len / 2)).unwrap() {
            Ordering::Less => "0",
            Ordering::Equal | Ordering::Greater => "1",
        }
    }).collect::<String>();
    let gamma = u32::from_str_radix(final_number.as_str(), 2).unwrap();
    let epsilon = 2u32.pow(report_item_len as u32) - 1 ^ gamma;
    println!("Task 1 result: {}", gamma * epsilon);

    // task 2
    let oxygen_rating = filter_candidates(&numbers, most_common);

    let co2_scrubber_rating = filter_candidates(&numbers, | numbers, pos | 1 ^ most_common(numbers, pos));

    println!("Task 2 result: {}", oxygen_rating * co2_scrubber_rating);
}
