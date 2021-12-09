// Copyright 2021 Michael Daum

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let lines = read_lines(data_file_path);

    let mut initial_positions: Vec<usize> = lines
        .into_iter()
        .flat_map(|s| {
            s.split(',')
                .map(|ns| ns.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    initial_positions.sort();

    let num_positions = initial_positions.len();
    let median =
        (initial_positions[(num_positions / 2) - 1] + initial_positions[num_positions / 2]) / 2;
    let linear_cost: i64 = initial_positions
        .iter()
        .map(|p| ((*p as i64) - (median as i64)).abs())
        .sum();
    println!("The linear_cost is {}", linear_cost);

    let sum: usize = Iterator::sum(initial_positions.iter());
    let mean: i64 = (sum as f64 / num_positions as f64).floor() as i64;
    let quadratic_cost: i64 = initial_positions
        .iter()
        .map(|p| {
            let d = (*p as i64 - mean).abs();
            d * (d + 1) / 2
        })
        .sum();
    println!("The quadratic_cost is {:?}", quadratic_cost);
}
