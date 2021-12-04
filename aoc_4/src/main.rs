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

type BingoBoard = Vec<Vec<i32>>;

// https://stackoverflow.com/a/64499219/7396293
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn check_bingo(board: &BingoBoard) -> bool {
    // check columns
    let result = board
        .iter()
        .map(|line| line.iter().all(|value| *value == -1))
        .any(|pred| pred);
    if result {
        return result;
    }
    let board = transpose(board.clone());
    let result = board
        .iter()
        .map(|line| line.iter().all(|value| *value == -1))
        .any(|pred| pred);
    return result;
}

fn calculate_bingo_sum(board: &BingoBoard) -> i32 {
    board
        .iter()
        .map(|row| row.iter().filter(|val| **val >= 0).sum::<i32>())
        .sum()
}

fn mark_board(board: &BingoBoard, draw: i32) -> BingoBoard {
    let result = board.clone();
    return result
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|val| if val == draw { -1 } else { val })
                .collect()
        })
        .collect();
}

fn task_1(mut boards: Vec<BingoBoard>, draws: &Vec<i32>) -> i32 {
    for draw in draws.iter() {
        boards = boards
            .iter()
            .map(|board| mark_board(board, *draw))
            .collect::<Vec<BingoBoard>>();
        let winning_boards = boards.iter().find(|board| check_bingo(board));
        match winning_boards {
            Some(board) => return draw * calculate_bingo_sum(board),
            None => {}
        };
    }
    panic!("Did not find any winning boards!");
}

fn task_2(mut boards: Vec<BingoBoard>, draws: &Vec<i32>) -> i32 {
    let mut result = -1;
    for draw in draws.iter() {
        boards = boards
            .iter()
            .map(|board| mark_board(board, *draw))
            .collect::<Vec<BingoBoard>>();
        let (winning_boards, open_boards) =
            boards.into_iter().partition(|board| check_bingo(board));
        boards = open_boards;
        for board in winning_boards.iter() {
            result = draw * calculate_bingo_sum(board);
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path);
    let bingo_draws: Vec<i32> = input[0]
        .split(",")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = input[1..].iter().fold(Vec::new(), |mut acc, line| {
        if line.trim().is_empty() {
            acc.push(Vec::new());
        } else {
            let cur_board = acc.last_mut().unwrap();
            let new_line: Vec<i32> = line
                .split(" ")
                .filter(|num| !num.is_empty())
                .map(|num| num.parse().unwrap())
                .collect();
            cur_board.push(new_line);
        }
        acc
    });
    println!("Task 1 result: {}", task_1(boards.clone(), &bingo_draws));
    println!("Task 2 result: {}", task_2(boards.clone(), &bingo_draws));
}
