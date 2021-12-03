use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut depth_readings = Vec::new();
    let mut prev_depth = 0;
    let mut prev_window = 0;
    let mut puzzle_one_counter = 0;
    let mut puzzle_two_counter = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let depth = line.parse::<u16>().unwrap();

        depth_readings.push(depth);

        if prev_depth > 0 && depth > prev_depth {
            puzzle_one_counter += 1;
        }

        if depth_readings.len() >= 3 {
            let sum: u16 = depth_readings.iter().rev().take(3).sum();
            if prev_window > 0 && sum > prev_window {
                puzzle_two_counter += 1;
            }
            prev_window = sum;
        }

        prev_depth = depth;
    }

    println!("Puzzle 1: {} Increases", puzzle_one_counter);
    println!("Puzzle 2: {} Increases", puzzle_two_counter);
}
