use itertools::Itertools;
use std::{collections::HashSet, iter::FromIterator};
use crate::utils;

pub fn day13(input_lines: &[String]) -> (u64, u64) {
    let (dots_input, folds_input) = utils::group_lines_split_by_empty_line(input_lines).into_iter().next_tuple().expect("Invalid input");
    let mut dots = parse_dots(dots_input);
    let folds = parse_folds(folds_input);

    perform_fold(&mut dots, folds[0]);
    let part1 = dots.len() as u64;

    folds[1..].iter().for_each(|&fold| perform_fold(&mut dots, fold));
    display_dots(&dots);

    (part1,0)
}

fn display_dots(dots: &HashSet<(usize, usize)>) {
    let cols = dots.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let rows = dots.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let mut display: Vec<Vec<char>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        display.push(std::iter::repeat(' ').take(cols).collect::<Vec<_>>());
    }
    for (col, row) in dots {
        display[*row][*col] = '#';
    }
    for row in display {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn perform_fold(dots: &mut HashSet<(usize, usize)>, fold: (char, usize)) {
    let index = if fold.0 == 'x' { 0usize } else { 1usize };
    let position = fold.1;
    let dots_copy = dots.clone();
    for (dot_x, dot_y) in dots_copy {
        let mut coords: [usize; 2] = [dot_x, dot_y];
        if coords[index] >= position {
            dots.remove(&(dot_x, dot_y));
            if coords[index] >= position {
                coords[index] -= (coords[index] - position) * 2;
                dots.insert((coords[0], coords[1]));
            }
        }
    }
}

fn parse_dots(dots_input: &[String]) -> HashSet<(usize, usize)> {
    HashSet::from_iter(
        dots_input
        .iter()
        .map(|line|
            line
            .split(',')
            .map(|number|
                number
                .parse::<usize>()
                .expect("Non-numeric input"))
            .tuples()
            .next()
            .expect("Invalid input")))
}

fn parse_folds(folds_input: &[String]) -> Vec<(char, usize)> {
    folds_input
        .iter()
        .map(|line|
            line
            .split_ascii_whitespace()
            .last()
            .expect("Invalid input")
            .split('=')
            .tuples::<(&str, &str)>()
            .map(|(direction, number)|
                (direction
                    .chars()
                    .next()
                    .unwrap(),
                number
                    .parse::<usize>()
                    .expect("Non-numeric fold instruction")))
            .next()
            .expect("Invalid input"))
        .collect()
}