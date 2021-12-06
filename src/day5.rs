use itertools::Itertools;
use std::cmp;

const FIELD_SIZE: usize = 1000;

pub fn day5(input_lines: &[String]) -> (u64, u64) {
    let mut locations: [[u8; FIELD_SIZE]; FIELD_SIZE] = [[0; FIELD_SIZE]; FIELD_SIZE];
    let part1 = add_and_calc_intersections(input_lines, &mut locations, false);
    let part2 = add_and_calc_intersections(input_lines, &mut locations, true);
    (part1, part2) 
}

fn add_and_calc_intersections(input_lines: &[String], locations: &mut [[u8; FIELD_SIZE]; FIELD_SIZE], diagonals: bool) -> u64 {
    for line in input_lines {
        let positions = parse_line(line, diagonals);
        for position in positions {
            locations[position.x][position.y] += 1;
        }
    }

    let mut intersections = 0u64;
    for (x, y) in (0..FIELD_SIZE).cartesian_product(0..FIELD_SIZE) {
        if locations[x][y] > 1 {
            intersections += 1;
        }
    }

    intersections
}

struct Position {
    x: usize,
    y: usize,
}

fn parse_line(line: &str, diagonals: bool) -> Vec<Position> {
    let mut tokens = line.split_whitespace();
    let start = parse_token(tokens.next().expect("Invalid input"));
    // Second token is the "->"
    tokens.next();
    let end = parse_token(tokens.next().expect("Invalid input"));

    let mut positions: Vec<Position> = Vec::new();
    if start.x == end.x {
        if !diagonals {
            for y in cmp::min(start.y, end.y)..=cmp::max(start.y, end.y) {
                positions.push(Position { x: start.x, y });
            }
        }
    } else if start.y == end.y {
        if !diagonals {
            for x in cmp::min(start.x, end.x)..=cmp::max(start.x, end.x) {
                positions.push(Position { x, y: start.y });
            }
        }
    } else if diagonals {
        let mut x = start.x as isize;
        let mut y = start.y as isize;
        let delta_x: isize = if end.x > start.x { 1 } else { -1 };
        let delta_y: isize = if end.y > start.y { 1 } else { -1 };
        let line_length = cmp::max(start.x, end.x) - cmp::min(start.x, end.x) + 1;
        for _ in 0..line_length {
            positions.push(Position { x: x as usize, y: y as usize });
            x += delta_x;
            y += delta_y;
        }
    }

    positions
}

fn parse_token(token: &str) -> Position {
    let mut coords = token.split(',');
    Position { 
        x: coords.next().expect("Invalid input").parse::<usize>().expect("Non-numeric co-ordinate"),
        y: coords.next().expect("Invalid input").parse::<usize>().expect("Non-numeric co-ordinate"),
    }
}