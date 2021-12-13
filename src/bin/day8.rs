use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
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

    let display_tuples: Vec<(Vec<String>, Vec<String>)> = tokens
        .into_iter()
        .map(|v| (v.iter().next().unwrap().clone(), v.iter().nth(1).unwrap().clone()))
        .collect();

    println!(
        "num_easy_identified  is {}",
        count_easily_identified(&display_tuples)
    );

    for (unique, _displays) in display_tuples {
        let mut token_map = HashMap::new();
		let mut char_map = HashMap::new();
        for token in &unique {
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
        let tokens_235: Vec<_> = unique.iter().filter(|s| s.len() == 5).collect();
        println!("235: {:?}", tokens_235);
        let tokens_069: Vec<_> = unique.iter().filter(|s| s.len() == 6).collect();
        println!("069: {:?}", tokens_069);
    }
}
