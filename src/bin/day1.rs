use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_numbers(file_name: &std::string::String) -> std::io::Result<Vec<i64>> {
    let data_file = File::open(file_name)?;
    let reader = BufReader::new(data_file);

    Ok(reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect())
}

fn num_single_increasing(numbers: &Vec<i64>) -> usize {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

fn sum3(numbers: &Vec<i64>) -> Vec<i64> {
    itertools::izip!(
        numbers.iter(),
        numbers.iter().skip(1),
        numbers.iter().skip(2)
    )
    .map(|(a, b, c)| a + b + c)
    .collect()
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let numbers = read_numbers(data_file_path)?;
    println!(
        "The number of increasing is {}",
        num_single_increasing(&numbers)
    );
    println!(
        "The number of increasing by three is {}",
        num_single_increasing(&sum3(&numbers))
    );
    Ok(())
}
