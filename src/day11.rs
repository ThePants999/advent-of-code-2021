use itertools::Itertools;

const GRID_SIZE: usize = 10;
const I_GRID_SIZE: isize = GRID_SIZE as isize;
const NUM_OCTOPUSES: usize = GRID_SIZE * GRID_SIZE;

pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let mut octopuses = [[Octopus::new(0); GRID_SIZE]; GRID_SIZE];
    for (row, line) in input_lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            octopuses[row][col] = Octopus::new(c.to_digit(10).expect("Invalid character in input"));
        }
    }

    let mut num_flashes = 0usize;
    let mut num_steps = 0u64;
    let mut part1 = 0u64;
    loop {
        num_steps += 1;
        let mut num_flashes_this_step = 0usize;

        for (row, col) in (0..GRID_SIZE).cartesian_product(0..GRID_SIZE) {
            let mut flashes: Vec<(isize, isize)> = Vec::new();
            if octopuses[row][col].increment() {
                num_flashes_this_step += 1;
                flashes.push((row as isize, col as isize));
                while let Some((flash_row, flash_col)) = flashes.pop() {
                    for (inc_row, inc_col) in (flash_row-1..=flash_row+1).cartesian_product(flash_col-1..=flash_col+1) {
                        if (0..I_GRID_SIZE).contains(&inc_row) && (0..I_GRID_SIZE).contains(&inc_col) && octopuses[inc_row as usize][inc_col as usize].increment() {
                            num_flashes_this_step += 1;
                            flashes.push((inc_row, inc_col));
                        }
                    }
                }
            }
        }

        num_flashes += num_flashes_this_step;
        if num_steps == 100 {
            part1 = num_flashes as u64;
        }
        if num_flashes_this_step == NUM_OCTOPUSES {
            break;
        }

        for (row, col) in (0..GRID_SIZE).cartesian_product(0..GRID_SIZE) {
            octopuses[row][col].reset();
        }
    }
    let part2 = num_steps;

    (part1,part2)
}

#[derive(Copy,Clone)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u32) -> Self {
        Self {
            energy,
            flashed: false,
        }
    }

    fn increment(&mut self) -> bool {
        if !self.flashed {
            self.energy += 1;
            if self.energy > 9 {
                self.energy = 0;
                self.flashed = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.flashed = false;
    }
}