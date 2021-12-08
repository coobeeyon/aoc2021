// Copyright 2021 Michael Daum

use itertools::Itertools;
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
    let segs = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|ns| ns.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|l| (l[0], l[1]))
                .collect::<Vec<_>>()
        })
        .map(|l| (l[0], l[1]))
        .collect::<Vec<_>>();

    let hv_segs = segs
        .iter()
        .filter(|((a, b), (c, d))| (a == c) || (b == d))
        .map(|((a, b), (c, d))| {
            if (a > c) || (b > d) {
                ((*c, *d), (*a, *b))
            } else {
                ((*a, *b), (*c, *d))
            }
        })
        .collect::<Vec<_>>();

    let hv_points: Vec<(i32, i32)> = hv_segs
        .iter()
        .flat_map(|((a, b), (c, d))| (*a..=*c).flat_map(move |i| (*b..=*d).map(move |j| (i, j))))
        .sorted_by(|a, b| Ord::cmp(b, a))
        .duplicates()
        .collect();
    println!("5a: there are {} overlapped points", hv_points.len());
}
