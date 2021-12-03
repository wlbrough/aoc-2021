use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut puzzle_one_depth: u32 = 0;
    let mut puzzle_two_depth: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let command = line.unwrap();
        let mut command_set = command.split_whitespace();
        let direction = command_set.next().unwrap();
        let magnitude_str = command_set.next().unwrap();
        let magnitude = magnitude_str.parse::<u32>().unwrap();

        if direction == "forward" {
            horizontal += magnitude;
            puzzle_two_depth += aim * magnitude;
        } else if direction == "down" {
            puzzle_one_depth += magnitude;
            aim += magnitude;
        } else if direction == "up" {
            puzzle_one_depth -= magnitude;
            aim -= magnitude;
        }
    }

    println!("Puzzle 1: {}", puzzle_one_depth * horizontal);
    println!("Puzzle 2: {}", puzzle_two_depth * horizontal);
}
