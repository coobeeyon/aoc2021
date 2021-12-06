use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_lines(file_name: &std::string::String) -> Vec<String> {
    let data_file = File::open(file_name).unwrap();
    let reader = BufReader::new(data_file);

    reader.lines().map(|r| r.unwrap()).collect()
}

type Board = Vec<Vec<(i32, bool)>>;
type Boards = Vec<Board>;

fn has_winning_row(board: &Board) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|(_, marked)| *marked))
}
fn has_winning_col(board: &Board) -> bool {
    let num_cols = board[0].len();
    (0..num_cols).any(|i| board.iter().map(|row| row[i]).all(|(_, marked)| marked))
}

fn is_winner(board: &Board) -> bool {
    has_winning_row(board) || has_winning_col(board)
}

fn mark_number(called_num: i32, board: &mut Board) {
    for row in board {
        for (num, found) in row {
            if called_num == *num {
                *found = true;
            }
        }
    }
}

fn score_board(called_num: i32, board: &Board) -> i32 {
    let mut sum = 0;
    for row in board {
        for (num, found) in row {
            if *found == false {
                sum += num
            }
        }
    }
    sum * called_num
}

fn read_boards(data_file_path: &String) -> (Vec<i32>, Boards) {
    let lines = read_lines(data_file_path);

    (
        lines[0].split(',').map(|ns| ns.parse::<i32>().unwrap()).collect(),
        lines
            .iter()
            .skip(1)
            .chunks(6)
            .into_iter()
            .map(|chunk| {
                chunk
                    .skip(1)
                    .map(|r| {
                        r.split_whitespace()
                            .map(|ns| (ns.parse::<i32>().unwrap(), false))
                            .collect()
                    })
                    .collect()
            })
            .collect(),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_file_path = &args[1];
    let (numbers, mut boards) = read_boards(data_file_path);

    let mut winners: Vec<(i32, Board)> = Vec::new();
    for n in numbers {
        for board in &mut boards {
            mark_number(n, board);
        }
        boards.retain(|board| {
            if is_winner(&board) {
                winners.push((n, board.clone()));
                false
            } else {
                true
            }
        });
    }

    if let Some((n, board)) = winners.first() {
        println!("The first winning score is {}", score_board(*n, &board));
    }
    if let Some((n, board)) = winners.last() {
        println!("The last winning score is {}", score_board(*n, &board));
    }
}
