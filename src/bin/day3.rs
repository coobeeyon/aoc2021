use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

#[derive(PartialEq, Debug)]
enum Bit {
    Zero,
    One,
}

fn invert(bits: &Vec<Bit>) -> Vec<Bit> {
    bits.iter()
        .map(|b| if *b == Bit::Zero { Bit::One } else { Bit::Zero })
        .collect()
}

fn evaluate(bits: &Vec<Bit>) -> i64 {
    bits.iter().fold(0, |val, bit| {
        if *bit == Bit::One {
            2 * val + 1
        } else {
            2 * val
        }
    })
}

fn calculate_majority_bits(vals: &Vec<Vec<Bit>>) -> Vec<Bit> {
    let num_bits = vals[0].len();
    let num_vals = vals.len();

    vals.iter()
        .fold(vec![0; num_bits], |a, b| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(n, m)| if *m == Bit::One { n + 1 } else { n })
                .collect::<Vec<i64>>()
        })
        .into_iter()
        .map(|n| {
            if (n * 2) as usize <= num_vals {
                Bit::Zero
            } else {
                Bit::One
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let lines = read_lines(data_file_path);
    let vals: Vec<Vec<Bit>> = lines
        .iter()
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .map(|byte| if *byte == b'0' { Bit::Zero } else { Bit::One })
                .collect()
        })
        .collect();

    let majority_bits = calculate_majority_bits(&vals);

    println!("majority_bits {:?}", majority_bits);
    let gamma = evaluate(&majority_bits);
    let epsilon = evaluate(&invert(&majority_bits));
    println!("values {} * {} = {}", gamma, epsilon, gamma * epsilon);
}
