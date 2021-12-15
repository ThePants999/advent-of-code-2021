use std::collections::BinaryHeap;

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    let mut cavern = parse_input(input_lines);
    let mut extended_cavern = cavern.extend();

    for row in extended_cavern.map.iter() {
        for node in row.iter() {
            print!("{}", node.cost);
        }
        println!();
    }

    let part1 = cavern.distance_top_left_to_bottom_right();
    let part2 = extended_cavern.distance_top_left_to_bottom_right();

    (part1, part2)
}

fn parse_input(input_lines: &[String]) -> Cavern {
    let rows = input_lines.len();
    let cols = input_lines[0].len();
    let mut map: Vec<Vec<Node>> = Vec::with_capacity(rows);

    for line in input_lines.iter() {
        let mut row: Vec<Node> = Vec::with_capacity(cols);
        for cost in line.chars().map(|c| c.to_digit(10).expect("Non-numeric input") as u64) {
            row.push(Node { cost, distance: u64::MAX, visited: false });
        }
        map.push(row);
    }

    Cavern::new(map, rows, cols)
}

struct Node {
    cost: u64,
    distance: u64,
    visited: bool,
}

#[derive(PartialEq, Eq)]
struct NodeDistance {
    distance: u64,
    row: usize,
    col: usize,
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // The reason we're implementing this manually is
        // because we need to compare the "wrong" way in
        // order to get our BinaryHeap ordered by *lowest*
        // cost.
        other.distance.cmp(&self.distance).then_with(|| self.row.cmp(&other.row)).then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Cavern {
    map: Vec<Vec<Node>>,
    heap: BinaryHeap<NodeDistance>,
    rows: usize,
    cols: usize,
}

impl Cavern {
    fn new(map: Vec<Vec<Node>>, rows: usize, cols: usize) -> Self {
        let mut cavern = Self { map, heap: BinaryHeap::with_capacity(rows * cols), rows, cols };
        let mut start = &mut cavern.map[0][0];
        start.distance = 0;
        cavern.heap.push(NodeDistance { distance: 0, row: 0, col: 0 });
        cavern
    }

    fn extend(&self) -> Self {
        let mut map: Vec<Vec<Node>> = Vec::with_capacity(self.rows * 5);
        for map_row in 0..5 {
            for original_row in 0..self.rows {
                let mut row = Vec::with_capacity(self.cols * 5);
                for map_col in 0..5 {
                    for original_col in 0..self.cols {
                        let node = &self.map[original_row][original_col];
                        let mut cost = node.cost + map_row as u64 + map_col as u64;
                        if cost > 9 { cost %= 9; }
                        row.push(Node { cost, distance: u64::MAX, visited: false });
                    }
                }
                map.push(row);
            }
        }
        Self::new(map, self.rows * 5, self.cols * 5)
    }

    fn distance_top_left_to_bottom_right(&mut self) -> u64 {
        let final_row = self.rows - 1;
        let final_col = self.cols - 1;

        while !self.map[final_row][final_col].visited {
            let next = self.heap.pop().unwrap();
            if !self.map[next.row][next.col].visited {
                self.visit(next.row, next.col);
            }
        }

        self.map[final_row][final_col].distance
    }

    fn visit(&mut self, row: usize, col: usize) {
        let distance = self.map[row][col].distance;

        if row > 0 {
            self.consider(row - 1, col, distance);
        }
        if row < self.rows - 1 {
            self.consider(row + 1, col, distance);
        }
        if col > 0 {
            self.consider(row, col - 1, distance);
        }
        if col < self.cols - 1 {
            self.consider(row, col + 1, distance);
        }

        self.map[row][col].visited = true;
    }

    fn consider(&mut self, row: usize, col: usize, from_distance: u64) {
        if !self.map[row][col].visited {
            let mut node = &mut self.map[row][col];
            let new_distance = from_distance + node.cost;
            if new_distance < node.distance {
                node.distance = new_distance;
                self.heap.push(NodeDistance { distance: new_distance, row, col });
            }
        }
    }
}