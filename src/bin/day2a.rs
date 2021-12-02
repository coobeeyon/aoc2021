use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Command {
    HorizontalMove(i64),
    DepthMove(i64),
    BadCommand,
}

#[derive(Debug)]
struct Pos(i64, i64);

fn parse_command(raw_string: String) -> Command {
    let words: Vec<&str> = raw_string.split(' ').take(2).collect();
    let (cmd, val) = (words[0], words[1].parse::<i64>().unwrap());
    match cmd {
        "forward" => Command::HorizontalMove(val),
        "up" => Command::DepthMove(-val),
        "down" => Command::DepthMove(val),
        _ => Command::BadCommand,
    }
}

fn read_commands(file_name: &std::string::String) -> Vec<Command> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader
        .lines()
        .map(|line| parse_command(line.unwrap()))
        .collect()
}

fn apply_command(command: Command, pos: Pos) -> Pos {
    match command {
        Command::HorizontalMove(val) => Pos(pos.0 + val, pos.1),
        Command::DepthMove(val) => Pos(pos.0, pos.1 + val),
        Command::BadCommand => Pos(0, 0),
    }
}

fn apply_commands(commands: Vec<Command>, pos: Pos) -> Pos {
    let mut cur_pos = pos;
    for command in commands {
        println!("position is {:?}", cur_pos);
        cur_pos = apply_command(command, cur_pos);
    }
    cur_pos
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let commands = read_commands(data_file_path);
    let final_pos = apply_commands(commands, Pos(0, 0));
    println!("Final position is {:?}", final_pos);
    println!("answer is {}", final_pos.0 * final_pos.1);
}
