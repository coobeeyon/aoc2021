use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

fn read_lines(file_name: &str) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

fn count_easily_identified(display_tuples: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    display_tuples
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .map(|s| s.len())
                .filter(|n| [2, 4, 3, 7].contains(n))
                .count()
        })
        .sum()
}

fn get_display_tuples(data_file_path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let lines = read_lines(data_file_path);
    let tokens: Vec<Vec<Vec<String>>> = lines
        .iter()
        .map(|l| {
            l.split('|')
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| String::from_iter(s.chars().sorted()))
                        .collect()
                })
                .collect()
        })
        .collect();

    tokens
        .into_iter()
        .map(|v| {
            (
                v.iter().next().unwrap().clone(),
                v.iter().nth(1).unwrap().clone(),
            )
        })
        .collect()
}

fn sum_entry(digits: Vec<String>, displays: Vec<String>) {
    let mut token_map = HashMap::new();
    let mut char_map = HashMap::new();

    for token in &digits {
        if let Some(digit) = match token.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        } {
            token_map.insert(digit, token.clone());
        }
    }

    for c in token_map.get(&7).unwrap().chars() {
        if !token_map.get(&1).unwrap().contains(c) {
            char_map.insert('a', c);
        }
    }
    println!("Here is the token map {:?}", token_map);
    println!("Here is the char map {:?}", char_map);
    let tokens_235: Vec<_> = digits.iter().filter(|s| s.len() == 5).collect();
    println!("235: {:?}", tokens_235);
    let tokens_069: Vec<_> = digits.iter().filter(|s| s.len() == 6).collect();
    println!("069: {:?}", tokens_069);
    println!("");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let display_tuples = get_display_tuples(&data_file_path);
    println!(
        "num_easy_identified  is {}",
        count_easily_identified(&display_tuples)
    );

    for (digits, displays) in display_tuples {
        sum_entry(digits, displays);
    }
}
