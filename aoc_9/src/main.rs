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

type MatrixPos = (usize, usize, u32);

fn get_neighbor_values_and_positions(
    matrix: &[Vec<u32>],
    pos_x: usize,
    pos_y: usize,
) -> Vec<MatrixPos> {
    let mut result = Vec::new();
    if pos_y > 0 {
        result.push((pos_x, pos_y - 1, matrix[pos_y - 1][pos_x]))
    }
    if pos_x > 0 {
        result.push((pos_x - 1, pos_y, matrix[pos_y][pos_x - 1]))
    }
    if pos_y < matrix.len() - 1 {
        result.push((pos_x, pos_y + 1, matrix[pos_y + 1][pos_x]))
    }
    if pos_x < matrix.first().unwrap().len() - 1 {
        result.push((pos_x + 1, pos_y, matrix[pos_y][pos_x + 1]))
    }
    result
}

fn all_greater_than(vector: &[u32], value: u32) -> bool {
    vector.iter().all(|&elem| value < elem)
}

fn get_sinks(matrix: &[Vec<u32>]) -> Vec<Vec<MatrixPos>> {
    matrix
        .iter()
        .enumerate()
        .map(|(pos_y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(pos_x, &item)| {
                    all_greater_than(
                        get_neighbor_values_and_positions(matrix, pos_x, pos_y)
                            .into_iter()
                            .map(|val| val.2)
                            .collect::<Vec<u32>>()
                            .as_slice(),
                        item,
                    )
                })
                .map(|(pos_x, &item)| (pos_x, pos_y, item))
                .collect()
        })
        .collect()
}

fn dfs(matrix: &mut [Vec<u32>], pos_x: usize, pos_y: usize) -> u32 {
    if matrix[pos_y][pos_x] > 8 {
        return 0;
    }
    matrix[pos_y][pos_x] = 9;
    let neighbors = get_neighbor_values_and_positions(matrix, pos_x, pos_y);
    neighbors
        .iter()
        .map(|neighbor| dfs(matrix, neighbor.0, neighbor.1))
        .sum::<u32>()
        + 1
}

fn task_2(matrix: &[Vec<u32>]) -> u32 {
    let mut matrix: Vec<Vec<u32>> = matrix.to_vec();
    let sinks = get_sinks(matrix.as_slice());
    let sinks: Vec<&MatrixPos> = sinks.iter().flatten().collect();
    let mut basin_sizes: Vec<u32> = sinks
        .into_iter()
        .map(|sink| dfs(matrix.as_mut(), sink.0, sink.1))
        .collect();
    basin_sizes.sort();
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

fn task_1(matrix: &[Vec<u32>]) -> u32 {
    let sinks: Vec<Vec<MatrixPos>> = get_sinks(matrix);
    let flat_vec: Vec<MatrixPos> = sinks.into_iter().flatten().collect();
    flat_vec.iter().map(|pos| pos.2).sum::<u32>() + flat_vec.len() as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input: Vec<Vec<u32>> = read(path)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();

    println!("Task 1 result: {}", task_1(input.as_slice()));
    println!("Task 2 result: {}", task_2(input.as_slice()));
}
