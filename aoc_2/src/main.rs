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

#[derive(Debug)]
struct Instruction {
    direction: String,
    value: i32,
}

#[derive(Debug)]
struct Location {
    depth: i32,
    horizontal: i32,
}

#[derive(Debug)]
struct State {
    location: Location,
    aim: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let instructions: Vec<Instruction> = read(path)
        .into_iter()
        .map(|line| {
            let line_split: Vec<&str> = line.split_whitespace().collect();
            Instruction {
                direction: String::from(line_split[0]),
                value: line_split[1].parse().expect("Has to be int"),
            }
        })
        .collect();
    let location_1 = instructions.iter().fold(
        Location {
            depth: 0,
            horizontal: 0,
        },
        |location, instruction| {
            match instruction {
                Instruction {
                    ref direction,
                    value,
                } if direction == "up" => Ok(Location {
                    depth: location.depth - value,
                    ..location
                }),
                Instruction {
                    ref direction,
                    value,
                } if direction == "down" => Ok(Location {
                    depth: location.depth + value,
                    ..location
                }),
                Instruction {
                    ref direction,
                    value,
                } if direction == "forward" => Ok(Location {
                    horizontal: location.horizontal + value,
                    ..location
                }),
                error => Err(error),
            }
            .unwrap()
        },
    );

    println!("Location Task 1: {:?}", location_1);
    println!(
        "Task 1 result: {}",
        location_1.horizontal * location_1.depth
    );
    let state_2 = instructions.iter().fold(
        State {
            location: Location {
                depth: 0,
                horizontal: 0,
            },
            aim: 0,
        },
        |state, instruction| {
            match instruction {
                Instruction {
                    ref direction,
                    value,
                } if direction == "up" => Ok(State {
                    aim: state.aim - value,
                    ..state
                }),
                Instruction {
                    ref direction,
                    value,
                } if direction == "down" => Ok(State {
                    aim: state.aim + value,
                    ..state
                }),
                Instruction {
                    ref direction,
                    value,
                } if direction == "forward" => Ok(State {
                    location: Location {
                        horizontal: state.location.horizontal + value,
                        depth: state.location.depth + state.aim * value,
                    },
                    ..state
                }),
                error => Err(error),
            }
            .unwrap()
        },
    );
    println!("State Task 2: {:?}", state_2);
    println!(
        "Task 2 result: {}",
        state_2.location.horizontal * state_2.location.depth
    );
}
