use std::cmp;
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

fn do_flash(matrix: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    for pos_y in y.saturating_sub(1)..cmp::min(y + 2, matrix.len()) {
        for pos_x in x.saturating_sub(1)..cmp::min(x + 2, matrix.first().unwrap().len()) {
            if pos_x != x || pos_y != y {
                matrix[pos_y][pos_x] += 1;
            }
        }
    }
}

fn do_step(matrix: &Vec<Vec<u8>>) -> (u32, Vec<Vec<u8>>) {
    let mut total_flashes = 0;
    let mut already_flashed: Vec<Vec<bool>> = matrix.iter().map(|v| vec![false; v.len()]).collect();
    let mut matrix: Vec<Vec<u8>> = matrix
        .iter()
        .map(|l| l.iter().map(|&v| v + 1).collect())
        .collect();

    loop {
        let mut round_flashes = 0;
        for y in 0..matrix.len() {
            for x in 0..matrix.first().unwrap().len() {
                if matrix[y][x] > 9 && !already_flashed[y][x] {
                    do_flash(matrix.as_mut(), x, y);
                    already_flashed[y][x] = true;
                    round_flashes += 1;
                }
            }
        }
        total_flashes += round_flashes;
        if round_flashes == 0 {
            break;
        }
    }
    already_flashed.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &has_flashed)| {
            if has_flashed {
                matrix[y][x] = 0;
            }
        });
    });
    (total_flashes, matrix)
}

fn task_1(matrix: &Vec<Vec<u8>>) -> u32 {
    let mut total_flashes = 0u32;
    let mut matrix = matrix.clone();
    for _ in 0..100 {
        let result = do_step(matrix.as_mut());
        total_flashes += result.0;
        matrix = result.1;
    }
    total_flashes
}

fn task_2(matrix: &Vec<Vec<u8>>) -> u32 {
    let all_flashed = (matrix.len() * matrix.first().unwrap().len()) as u32;
    let mut matrix = matrix.clone();
    let mut step = 1;
    loop {
        let result = do_step(matrix.as_mut());
        matrix = result.1;
        if result.0 == all_flashed {
            break;
        }
        step += 1;
    }
    step
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input: Vec<Vec<u8>> = read(path)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    println!("Task 1 result: {}", task_1(&input));
    println!("Task 2 result: {}", task_2(&input));
}
