use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
}

fn get_range_inclusive(start: u32, end: u32) -> (u32, u32) {
    if start <= end {
        (start, end + 1)
    } else {
        (end, start + 1)
    }
}

fn get_range_iterator_inclusive(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start <= end {
        Box::new(start..end + 1)
    } else {
        Box::new((end..start + 1).rev())
    }
}

fn calculate_horizontal_vertical_lines(result_map: &mut HashMap<Point, u32>, line: &Line) {
    // ranges not created since they have to be created multiple times
    let (x1, x2) = get_range_inclusive(line.p1.x, line.p2.x);
    let (y1, y2) = get_range_inclusive(line.p1.y, line.p2.y);

    for x in x1..x2 {
        for y in y1..y2 {
            let entry = result_map.entry(Point { x, y }).or_insert(0);
            *entry += 1;
        }
    }
}

fn calculate_diagonal_lines(result_map: &mut HashMap<Point, u32>, line: &Line) {
    // boxing necessary to avoid type mismatch between Rev and Range
    let range_x = get_range_iterator_inclusive(line.p1.x, line.p2.x);
    let range_y = get_range_iterator_inclusive(line.p1.y, line.p2.y);
    let iterator = range_x.zip(range_y);
    for (x, y) in iterator {
        let entry = result_map.entry(Point { x, y }).or_insert(0);
        *entry += 1;
    }
}

fn calculate_score(result_map: HashMap<Point, u32>) -> u32 {
    result_map
        .into_iter()
        .map(|(_, value)| if value > 1 { 1 } else { 0 })
        .sum()
}

fn task_1(lines: &Vec<Line>) -> u32 {
    let filtered_lines: Vec<&Line> = lines
        .iter()
        .filter(|line| line.is_horizontal_or_vertical())
        .collect();
    let point_map: HashMap<Point, u32> =
        filtered_lines
            .into_iter()
            .fold(HashMap::new(), |mut acc, line| {
                calculate_horizontal_vertical_lines(acc.borrow_mut(), line);
                acc
            });
    calculate_score(point_map)
}

fn task_2(lines: &Vec<Line>) -> u32 {
    let point_map: HashMap<Point, u32> = lines.into_iter().fold(HashMap::new(), |mut acc, line| {
        if line.is_horizontal_or_vertical() {
            calculate_horizontal_vertical_lines(acc.borrow_mut(), line);
        } else {
            calculate_diagonal_lines(acc.borrow_mut(), line);
        }
        acc
    });
    calculate_score(point_map)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path);
    let lines: Vec<Line> = input
        .iter()
        .map(|line| {
            let mut points: Vec<Point> = line
                .split(" -> ")
                .map(|p| {
                    let split: Vec<u32> =
                        p.split(",").map(|coord| coord.parse().unwrap()).collect();
                    Point {
                        x: split[0],
                        y: split[1],
                    }
                })
                .collect();
            let p2 = points.pop().unwrap();
            let p1 = points.pop().unwrap();
            Line { p1, p2 }
        })
        .collect();
    println!("Task 1 result: {}", task_1(&lines));
    println!("Task 2 result: {}", task_2(&lines));
}
