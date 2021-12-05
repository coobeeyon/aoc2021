use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

#[derive(PartialEq, Debug, Clone)]
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
            if ((n * 2) as usize) < num_vals {
                Bit::Zero
            } else {
                Bit::One
            }
        })
        .collect()
}

fn calculate_rating(bits: &Vec<Bit>, vals: &Vec<Vec<Bit>>, should_invert:bool) -> i64 {
    let mut filtered_vals = vals.to_vec();
	let mut filtered_bits = bits.to_vec();
	let mut i = 0;
    while filtered_vals.len() > 1 {
        filtered_vals = filtered_vals.into_iter().filter(|v| v[i] == filtered_bits[i]).collect();
		filtered_bits  = calculate_majority_bits(&filtered_vals);
		if should_invert {filtered_bits = invert(&filtered_bits)};
		i += 1;
    }
    evaluate(&filtered_vals[0])
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
    let minority_bits = invert(&majority_bits);

    println!("majority_bits {:?}", majority_bits);
    let gamma = evaluate(&majority_bits);
    let epsilon = evaluate(&minority_bits);
    println!("values {} * {} = {}", gamma, epsilon, gamma * epsilon);

	let oxygen = calculate_rating(&majority_bits, &vals, false);
	let co2 = calculate_rating(&minority_bits, &vals, true);
	println!("ratings {} * {} = {}", oxygen, co2, oxygen * co2);
}
