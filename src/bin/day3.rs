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
    let vals: Vec<_> = lines
        .iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|byte| byte - b'0')
                .collect::<Vec<_>>()
        })
        .reduce(|a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(n, m)| n + m)
                .collect::<Vec<u8>>()
        });
}
