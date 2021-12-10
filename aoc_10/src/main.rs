use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};

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

fn score_invalid(parentheses: char) -> u64 {
    match parentheses {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid char"),
    }
}

fn score_autocomplete(parentheses: char) -> u64 {
    match parentheses {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid char"),
    }
}

fn get_closing_parentheses(parentheses: &char) -> char {
    match parentheses {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid char"),
    }
}

fn find_first_invalid_parentheses(line: &[char]) -> Option<char> {
    let mut parentheses_stack = Vec::new();
    for c in line {
        match c {
            x if ['{', '<', '(', '['].contains(x) => parentheses_stack.push(x),
            &y => {
                if y != get_closing_parentheses(parentheses_stack.pop().unwrap()) {
                    return Option::Some(y);
                }
            }
        }
    }
    Option::None
}

fn calculate_autocomplete_score(line: &[char]) -> u64 {
    let parentheses_stack = line.iter().fold(Vec::new(), | mut acc ,c | {
        match c {
            x if ['{', '<', '(', '['].contains(x) => {
                acc.push(x);
            }
            _ => {
                get_closing_parentheses(acc.pop().unwrap());
            }
        }
        acc
    });
    parentheses_stack
        .into_iter()
        .map(|char| get_closing_parentheses(char))
        .rev()
        .fold(0, |acc, item| acc * 5 + score_autocomplete(item))
}

fn task_1(input: &Vec<Vec<char>>) -> u64 {
    input
        .iter()
        .map(|l| {
            let val = find_first_invalid_parentheses(l);
            val.map(|m| score_invalid(m)).unwrap_or_default()
        })
        .sum()
}

fn task_2(input: &Vec<Vec<char>>) -> u64 {
    let incomplete_lines: Vec<&Vec<char>> = input
        .into_iter()
        .filter(|&line| find_first_invalid_parentheses(line).is_none())
        .collect();
    let mut line_autocomplete_values: Vec<u64> = incomplete_lines
        .into_iter()
        .map(|line| calculate_autocomplete_score(line))
        .collect();
    line_autocomplete_values.sort();
    line_autocomplete_values[line_autocomplete_values.len() / 2]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input: Vec<Vec<char>> = read(path)
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("Result Task 1: {}", task_1(&input));
    println!("Result Task 2: {}", task_2(&input));
}
