use itertools::Itertools;
use std::collections::HashSet;

pub fn day9(input_lines: &[String]) -> (u64, u64) {
    let heights = parse_input(input_lines);
    let rows = heights.len();
    let cols = heights[0].len();

    let mut low_points: Vec<Position> = Vec::new();
    for (row, col) in (0..rows).cartesian_product(0..cols) {
        let height = heights[row][col];
        if (row == 0 || height < heights[row-1][col]) &&
            (row == rows-1 || height < heights[row+1][col]) &&
            (col == 0 || height < heights[row][col-1]) &&
            (col == cols-1 || height < heights[row][col+1]) {
            low_points.push(Position { row, col, height });
        }
    }
    let part1 = low_points.iter().map(|pos| pos.height).sum::<u64>() + low_points.len() as u64;

    let mut basins: Vec<u64> = Vec::with_capacity(low_points.len());
    for lp in low_points {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        basin.insert((lp.row, lp.col));

        let mut unexplored_locations: Vec<(usize, usize)> = vec![(lp.row, lp.col)];
        while !unexplored_locations.is_empty() {
            let (row, col) = unexplored_locations.pop().unwrap();

            if row > 0 && !basin.contains(&(row-1, col)) && heights[row-1][col] != 9 {
                let loc = (row-1, col);
                basin.insert(loc);
                unexplored_locations.push(loc);
            }
            if row < rows-1 && !basin.contains(&(row+1, col)) && heights[row+1][col] != 9 {
                let loc = (row+1, col);
                basin.insert(loc);
                unexplored_locations.push(loc);
            }
            if col > 0 && !basin.contains(&(row, col-1)) && heights[row][col-1] != 9 {
                let loc = (row, col-1);
                basin.insert(loc);
                unexplored_locations.push(loc);
            }
            if col < cols-1 && !basin.contains(&(row, col+1)) && heights[row][col+1] != 9 {
                let loc = (row, col+1);
                basin.insert(loc);
                unexplored_locations.push(loc);
            }
        }

        basins.push(basin.len() as u64);
    }
    basins.sort_unstable();
    let part2 = basins.iter().rev().take(3).product::<u64>();

    (part1,part2)
}

fn parse_input(input_lines: &[String]) -> Vec<Vec<u64>> {
    let mut heights_outer: Vec<Vec<u64>> = Vec::with_capacity(input_lines.len());
    for line in input_lines {
        let mut heights_inner = Vec::with_capacity(line.len());
        for height in line.chars().map(|c| c.to_digit(10).expect("Non-numeric input") as u64) {
            heights_inner.push(height);
        }
        heights_outer.push(heights_inner);
    }
    heights_outer
}

struct Position {
    row: usize,
    col: usize,
    height: u64,
}