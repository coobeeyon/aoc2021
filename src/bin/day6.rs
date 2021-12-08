// Copyright 2021 Michael Daum

use itertools::iterate;
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

    let fish_days: Vec<i32> = lines
        .into_iter()
        .flat_map(|s| {
            s.split(',')
                .map(|ns| ns.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let ddays: Vec<Vec<_>> = iterate(fish_days, |days| {
        days.iter()
            .flat_map(|day| match day {
                0 => vec![6, 8],
                _ => vec![day - 1],
            })
            .collect()
    })
    .take(81)
    .collect();

    println!("ddays is {:?}", ddays.last().unwrap().len());
}
