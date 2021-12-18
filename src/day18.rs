use itertools::Itertools;

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let numbers = parse_input(input_lines);
    let mut numbers_iter = numbers.iter();
    let first = numbers_iter.next().unwrap().clone();
    let part1 = numbers_iter.fold(first, |acc, number| SFNumber::add(&acc, number)).magnitude();
    let part2 = numbers.into_iter().combinations(2).map(|nums| SFNumber::add(&nums[0], &nums[1]).magnitude()).max().unwrap();
    (part1, part2)
}

fn parse_input(input_lines: &[String]) -> Vec<SFNumber> {
    let mut numbers: Vec<SFNumber> = Vec::with_capacity(input_lines.len());
    for line in input_lines {
        let mut numbers_stack: Vec<SFNumberUnderConstruction> = Vec::new();
        for c in line.chars() {
            match c {
                '[' => numbers_stack.push(SFNumberUnderConstruction::new()),
                ']' => {
                    let finished_number = numbers_stack.pop().unwrap();
                    if let Some(parent_number) = numbers_stack.last_mut() {
                        parent_number.add_item(SFElementUnderConstruction::Pair(Box::new(finished_number)));
                    } else {
                        // This was the top-level number.
                        numbers.push(finished_number.final_form());
                    }
                },
                ',' => (),
                _ => {
                    let num = c.to_digit(10).expect("Invalid input") as u64;
                    numbers_stack.last_mut().unwrap().add_item(SFElementUnderConstruction::Single(num));
                }
            }
        }
    }
    numbers
}

struct SFNumberUnderConstruction {
    first: Option<SFElementUnderConstruction>,
    second: Option<SFElementUnderConstruction>,
}

impl SFNumberUnderConstruction {
    fn new() -> Self {
        Self { first: None, second: None }
    }

    fn add_item(&mut self, element: SFElementUnderConstruction) {
        if self.first.is_none() {
            self.first = Some(element);
        } else {
            assert!(self.second.is_none());
            self.second = Some(element);
        }
    }

    fn final_form(self) -> SFNumber {
        SFNumber {
            first: self.first.unwrap().final_form(),
            second: self.second.unwrap().final_form(),
        }
    }
}

enum SFElementUnderConstruction {
    Single(u64),
    Pair(Box<SFNumberUnderConstruction>),
}

impl SFElementUnderConstruction {
    fn final_form(self) -> SFElement {
        match self {
            Self::Single(num) => SFElement::Single(num),
            Self::Pair(pair) => SFElement::Pair(Box::new(pair.final_form())),
        }
    }
}

#[derive(Clone)]
struct SFNumber {
    first: SFElement,
    second: SFElement,
}

impl SFNumber {
    fn add(lhs: &Self, rhs: &Self) -> Self {
        SFNumber {
            first: SFElement::Pair(Box::new(lhs.clone())),
            second: SFElement::Pair(Box::new(rhs.clone())),
        }.reduce()
    }

    fn reduce(mut self) -> Self {
        loop {
            match self.check_for_explosion(0) {
                ExplosionResult::None => {
                    match self.check_for_split() {
                        SplitResult::None => break,
                        SplitResult::Handled => (),
                        SplitResult::Unhandled(_) => unreachable!("Split was unhandled"),
                    }
                },
                ExplosionResult::Handled | ExplosionResult::LeftUnhandled(_) | ExplosionResult::RightUnhandled(_) => (),
                ExplosionResult::Unhandled(_, _) => unreachable!("Explosion was unhandled"),
            }
        }
        self
    }

    fn check_for_explosion(&mut self, recursion_level: usize) -> ExplosionResult {
        if recursion_level == 4 {
            // Explode. Hopefully we consist of singles!
            if let SFElement::Single(left) = self.first {
                if let SFElement::Single(right) = self.second {
                    ExplosionResult::Unhandled(left, right)
                } else {
                    unreachable!("Mid-level number exploded");
                }
            } else {
                unreachable!("Mid-level number exploded");
            }
        } else {
            // Recurse.
            let mut result = self.first.check_for_explosion(recursion_level + 1);
            match result {
                ExplosionResult::Handled => (),
                ExplosionResult::None => (),
                ExplosionResult::Unhandled(left, right) => {
                    // An explosion on the left element means that we can definitely handle
                    // the right, but need to return the left.
                    self.second.explode_right(right);
                    result = ExplosionResult::LeftUnhandled(left);
                    self.first = SFElement::Single(0);
                },
                ExplosionResult::LeftUnhandled(_) => (),
                ExplosionResult::RightUnhandled(right) => {
                    self.second.explode_right(right);
                    result = ExplosionResult::Handled;
                },
            }

            if matches!(result, ExplosionResult::None) {
                result = self.second.check_for_explosion(recursion_level + 1);

                match result {
                    ExplosionResult::Handled => (),
                    ExplosionResult::None => (),
                    ExplosionResult::Unhandled(left, right) => {
                        // An explosion on the right element means that we can definitely handle
                        // the left, but need to return the right.
                        self.first.explode_left(left);
                        result = ExplosionResult::RightUnhandled(right);
                        self.second = SFElement::Single(0);
                    },
                    ExplosionResult::LeftUnhandled(left) => {
                        self.first.explode_left(left);
                        result = ExplosionResult::Handled;
                    },
                    ExplosionResult::RightUnhandled(_) => (),
                }
            }

            result
        }
    }

    fn check_for_split(&mut self) -> SplitResult {
        let mut result = self.first.check_for_split();
        match result {
            SplitResult::None => {
                result = self.second.check_for_split();
                if let SplitResult::Unhandled(new_element) = result {
                    self.second = new_element;
                    result = SplitResult::Handled;
                }
            },
            SplitResult::Handled => {},
            SplitResult::Unhandled(new_element) => {
                self.first = new_element;
                result = SplitResult::Handled;
            }
        }
        result
    }

    fn magnitude(&self) -> u64 {
        self.first.magnitude() * 3 + self.second.magnitude() * 2
    }
}

enum ExplosionResult {
    None,
    Unhandled(u64, u64),
    LeftUnhandled(u64),
    RightUnhandled(u64),
    Handled,
}

enum SplitResult {
    None,
    Unhandled(SFElement),
    Handled,
}

impl std::fmt::Display for SFNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.first, self.second)
    }
}

#[derive(Clone)]
enum SFElement {
    Single(u64),
    Pair(Box<SFNumber>),
}

impl SFElement {
    fn magnitude(&self) -> u64 {
        match self {
            Self::Single(num) => *num,
            Self::Pair(pair) => pair.magnitude(),
        }
    }

    fn check_for_explosion(&mut self, recursion_level: usize) -> ExplosionResult {
        match self {
            Self::Single(_) => ExplosionResult::None,
            Self::Pair(pair) => pair.check_for_explosion(recursion_level),
        }
    }

    fn check_for_split(&mut self) -> SplitResult {
        match self {
            Self::Single(num) => {
                if *num > 9 {
                    // Split
                    let left_num = *num / 2;
                    let left = SFElement::Single(left_num);
                    let right = SFElement::Single(*num - left_num);
                    SplitResult::Unhandled(SFElement::Pair(Box::new(SFNumber { first: left, second: right })))
                } else {
                    SplitResult::None
                }
            },
            Self::Pair(pair) => pair.check_for_split()
        }
    }

    fn explode_left(&mut self, number: u64) {
        match self {
            Self::Single(num) => *num += number,
            Self::Pair(pair) => pair.second.explode_left(number),
        }
    }

    fn explode_right(&mut self, number: u64) {
        match self {
            Self::Single(num) => *num += number,
            Self::Pair(pair) => pair.first.explode_right(number),
        }
    }
}

impl std::fmt::Display for SFElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(num) => write!(f, "{}", num),
            Self::Pair(pair) => write!(f, "{}", pair),
        }
    }
}