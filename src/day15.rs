use std::collections::{HashMap, HashSet, BinaryHeap};

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    let mut cavern = parse_input(input_lines);
    let mut extended_cavern = cavern.extend();
    let part1 = cavern.distance_top_left_to_bottom_right();
    let part2 = extended_cavern.distance_top_left_to_bottom_right();

    (part1, part2)
}

fn parse_input(input_lines: &[String]) -> Cavern {
    let rows = input_lines.len() as isize;
    let cols = input_lines[0].len() as isize;
    let nodes = (rows * cols) as usize;
    let mut map: HashMap<(isize, isize), Node> = HashMap::with_capacity(nodes);

    for (row, line) in input_lines.iter().enumerate() {
        for (col, cost) in line.chars().map(|c| c.to_digit(10).expect("Non-numeric input") as u64).enumerate() {
            map.insert((row as isize, col as isize), Node { cost, distance: u64::MAX });
        }
    }

    Cavern::new(map, rows, cols)
}

struct Node {
    cost: u64,
    distance: u64,
}

#[derive(PartialEq, Eq)]
struct NodeDistance {
    distance: u64,
    row: isize,
    col: isize,
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
    map: HashMap<(isize, isize), Node>,
    visited: HashSet<(isize, isize)>,
    heap: BinaryHeap<NodeDistance>,
    rows: isize,
    cols: isize,
}

impl Cavern {
    fn new(map: HashMap<(isize, isize), Node>, rows: isize, cols: isize) -> Self {
        let nodes = (rows * cols) as usize;
        let mut cavern = Self { map, visited: HashSet::with_capacity(nodes), heap: BinaryHeap::with_capacity(nodes), rows, cols };
        let mut start = cavern.map.get_mut(&(0, 0)).unwrap();
        cavern.heap.push(NodeDistance { distance: 0, row: 0, col: 0 });
        start.distance = 0;
        cavern
    }

    fn extend(&self) -> Self {
        let nodes = (self.rows * self.cols * 25) as usize;
        let mut map: HashMap<(isize, isize), Node> = HashMap::with_capacity(nodes);
        for ((original_row, original_col), node) in self.map.iter() {
            for map_row in 0..5 {
                for map_col in 0..5 {
                    let row = original_row + (self.rows * map_row);
                    let col = original_col + (self.cols * map_col);
                    let mut cost = node.cost + map_row as u64 + map_col as u64;
                    if cost > 9 { cost %= 9; }
                    map.insert((row, col), Node { cost, distance: u64::MAX });
                }
            }
        }
        Self::new(map, self.rows * 5, self.cols * 5)
    }

    fn distance_top_left_to_bottom_right(&mut self) -> u64 {
        let final_coords = (self.rows - 1, self.cols - 1);

        while !self.visited.contains(&final_coords) {
            let next = self.heap.pop().unwrap();
            if !self.visited.contains(&(next.row, next.col)) {
                self.visit(next.row, next.col);
            }
        }

        self.map.get(&final_coords).unwrap().distance
    }

    fn visit(&mut self, row: isize, col: isize) {
        let distance = self.map.get(&(row, col)).unwrap().distance;

        self.consider(row - 1, col, distance);
        self.consider(row + 1, col, distance);
        self.consider(row, col - 1, distance);
        self.consider(row, col + 1, distance);

        self.visited.insert((row, col));
    }

    fn consider(&mut self, row: isize, col: isize, from_distance: u64) {
        if row >= 0 && row < self.rows && col >= 0 && col < self.rows && !self.visited.contains(&(row, col)) {
            let mut node = self.map.get_mut(&(row, col)).unwrap();
            let new_distance = from_distance + node.cost;
            if new_distance < node.distance {
                node.distance = new_distance;
                self.heap.push(NodeDistance { distance: new_distance, row, col });
            }
        }
    }
}