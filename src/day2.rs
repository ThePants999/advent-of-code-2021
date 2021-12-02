use std::str::FromStr;

pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let instructions: Vec<Instruction> = input_lines.iter().map(|line| Instruction::parse_line(line)).collect();
    let part1 = instructions.iter().fold(SimplePosition::default(), |position, instruction| position + instruction).product();
    let part2 = instructions.iter().fold(ComplexPosition::default(), |position, instruction| position + instruction).product();
    (part1, part2)
}

trait Position {
    fn product(&self) -> u64;
}

#[derive(Default)]
struct SimplePosition {
    horizontal: u64,
    depth: u64,
}

impl Position for SimplePosition {
    fn product(&self) -> u64 {
        self.horizontal * self.depth
    }
}

#[derive(Default)]
struct ComplexPosition {
    position: SimplePosition,
    aim: u64,
}

impl Position for ComplexPosition {
    fn product(&self) -> u64 {
        self.position.product()
    }
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(())
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: u64,
}

impl Instruction {
    fn parse_line(input_line: &str) -> Self {
        let pieces = input_line.split(' ').collect::<Vec<_>>();
        Self {
            direction: Direction::from_str(pieces[0]).expect("Direction could not be parsed from input"),
            distance: pieces[1].parse::<u64>().expect("Distance could not be parsed from input"),
        }
    }
}

impl std::ops::Add<&Instruction> for SimplePosition {
    type Output = Self;

    fn add(self, other: &Instruction) -> Self {
        match other.direction {
            Direction::Forward => Self { horizontal: self.horizontal + other.distance, depth: self.depth },
            Direction::Up => Self { horizontal: self.horizontal, depth: self.depth - other.distance },
            Direction::Down => Self { horizontal: self.horizontal, depth: self.depth + other.distance },
        }
    }
}

impl std::ops::Add<&Instruction> for ComplexPosition {
    type Output = Self;

    fn add(self, other: &Instruction) -> Self {
        match other.direction {
            Direction::Forward => Self { position: SimplePosition { horizontal: self.position.horizontal + other.distance, depth: self.position.depth + (self.aim * other.distance) }, aim: self.aim },
            Direction::Up => Self { position: self.position, aim: self.aim - other.distance },
            Direction::Down => Self { position: self.position, aim: self.aim + other.distance },
        }
    }
}