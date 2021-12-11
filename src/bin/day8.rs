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
    let tokens: Vec<Vec<Vec<_>>> = lines
        .iter()
        .map(|l| {
            l.split('|')
                .map(|s| s.split_whitespace().collect())
                .collect()
        })
        .collect();

    let display_tuples: Vec<(&Vec<_>, &Vec<_>)> = tokens
        .iter()
        .map(|v| (v.iter().next().unwrap(), v.iter().nth(1).unwrap()))
        .collect();
    let num_easy_identified: usize = display_tuples
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .map(|s| s.len())
                .filter(|n| [2, 4, 3, 7].contains(n))
                .count()
        })
        .sum();
    println!("num_easy_identified  is {}", num_easy_identified);
}
