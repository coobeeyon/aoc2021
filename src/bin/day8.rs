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
    let uniques = lines
        .iter()
        .map(|l| l.split('|').nth(1).unwrap().split_whitespace());
    let n: usize = uniques
        .map(|v| {
            v.map(|s| s.len())
                .filter(|n| [2, 4, 3, 7].contains(n))
                .count()
        })
        .sum();
    println!("n 1,4,7,8 = {}", n);
}
