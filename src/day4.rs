use std::collections::HashMap;

use crate::utils;

const BOARD_SIZE: usize = 5;

pub fn day4(input_lines: &[String]) -> (u64, u64) {
    let input_parts = utils::group_lines_split_by_empty_line(input_lines);
    let mut input_parts_iter = input_parts.iter();
    let called_numbers: Vec<u64> = input_parts_iter.next().expect("Empty input")[0].split(',').map(|num| num.parse::<u64>().expect("Non-numeric input found")).collect();
    let mut boards: HashMap<usize, Board> = input_parts_iter.map(|board_input| parse_board(*board_input)).enumerate().collect();

    let mut part1: Option<u64> = None;
    let mut part2 = 0u64;
    for number in called_numbers {
        let mut completed_boards: Vec<usize> = Vec::new();
        for (board_index, board) in boards.iter_mut() {
            if let Some(sum_of_remaining_numbers) = board.mark_number(number) {
                completed_boards.push(*board_index);
                let score = sum_of_remaining_numbers * number;
                if part1.is_none() {
                    part1 = Some(score);
                } else {
                    part2 = score;
                }
            }
        }
        for board_index in completed_boards {
            boards.remove(&board_index);
        }
    }
    (part1.unwrap(), part2)
}

fn parse_board(input_lines: &[String]) -> Board {
    let mut board = Board::default();
    for (row_num, row) in input_lines.iter().enumerate() {
        let numbers: Vec<u64> = row.split_whitespace().map(|num| num.parse::<u64>().expect("Non-numeric input found")).collect();
        for (col_num, number) in numbers.iter().enumerate() {
            board.numbers.insert(*number, Number { row: row_num, col: col_num });
        }
    }
    board
}

struct Number {
    row: usize,
    col: usize,
}

#[derive(Default)]
struct Board {
    numbers: HashMap<u64, Number>,
    marked_by_row: [usize; BOARD_SIZE],
    marked_by_col: [usize; BOARD_SIZE],
}

impl Board {
    fn mark_number(&mut self, number: u64) -> Option<u64> {
        if let Some(num) = self.numbers.remove(&number) {
            self.marked_by_row[num.row] += 1;
            self.marked_by_col[num.col] += 1;
            if (self.marked_by_row[num.row] == BOARD_SIZE) || (self.marked_by_col[num.col] == BOARD_SIZE) {
                Some(self.numbers.keys().sum())
            } else {
                None
            }
        } else {
            None
        }
    }
}