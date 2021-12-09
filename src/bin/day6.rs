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

    let initial_pop: Vec<usize> = lines
        .into_iter()
        .flat_map(|s| {
            s.split(',')
                .map(|ns| ns.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut fish_pop: Vec<u64> = vec![0; 9];
    initial_pop.into_iter().for_each(|n| fish_pop[n] += 1);

    let pop: Vec<_> = iterate(fish_pop, |l| {
        let mut ln = l.clone();
        ln.rotate_left(1);
        ln[6] += ln[8];
        ln
    })
    .take(257)
    .collect();

    println!(
        "After 80 days there are {} fish",
        pop[80].iter().sum::<u64>()
    );
    println!(
        "After 256 days there are {} fish",
        pop[256].iter().sum::<u64>()
    );
}
