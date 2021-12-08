// Copyright 2021 Michael Daum

use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

type Point = (i32, i32);
type Seg = (Point, Point);

fn numerical_cmp(a: &i32, b: &i32) -> i32 {
    match Ord::cmp(a, b) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

fn num_overlapping_hv(segs: &Vec<Seg>) -> usize {
    segs.iter()
        .filter(|((a, b), (c, d))| (a == c) || (b == d))
        .map(|((a, b), (c, d))| {
            if (a > c) || (b > d) {
                ((*c, *d), (*a, *b))
            } else {
                ((*a, *b), (*c, *d))
            }
        })
        .flat_map(|((a, b), (c, d))| (a..=c).flat_map(move |i| (b..=d).map(move |j| (i, j))))
        .sorted_by(|a, b| Ord::cmp(b, a))
        .duplicates()
        .collect::<Vec<Point>>()
        .len()
}

fn num_overlapping(segs: &Vec<Seg>) -> usize {
    segs.iter()
        .flat_map(|((a, b), (c, d))| {
            let (dx, dy) = (numerical_cmp(a, c), numerical_cmp(b, d));
            let mut points: Vec<Point> = <Vec<Point>>::new();
            let mut cur_point = (*a, *b);
            loop {
                points.push(cur_point.clone());
                if cur_point == (*c, *d) {
                    break;
                };
                let (a, b) = cur_point;
                cur_point = (a + dx, b + dy);
            }
            points
        })
        .sorted_by(|a, b| Ord::cmp(b, a))
        .duplicates()
        .collect::<Vec<Point>>()
        .len()
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

    println!(
        "5a: there are {} overlapped hv points",
        num_overlapping_hv(&segs)
    );
    println!("5a: there are {} overlapped points", num_overlapping(&segs));
}
