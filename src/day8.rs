use itertools::Itertools;
use std::collections::{HashSet, HashMap};

pub fn day8(input_lines: &[String]) -> (u64, u64) {
    let displays: Vec<SubDisplay> = input_lines.iter().map(|line| SubDisplay::parse_and_solve(line)).collect();
    let part1: u64 = displays.iter().map(|display| display.digits.iter().filter(|&&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8).count() as u64).sum();
    let part2: u64 = displays.iter().map(|display| display.value).sum();
    (part1,part2)
}

struct Digit {
    set: HashSet<char>,
    string: String,
}

impl Digit {
    fn parse(input: &str) -> Self {
        let mut set = HashSet::with_capacity(input.len());
        set.extend(input.chars());
        let string = input.chars().sorted().collect();
        Self {
            set,
            string,
        }
    }
}

struct SubDisplay {
    digits: Vec<u64>,
    value: u64,
}

impl SubDisplay {
    fn parse_and_solve(input_line: &str) -> Self {
        let mut sections = input_line.split(" | ");
        let inputs: Vec<Digit> = sections.next().unwrap().split_ascii_whitespace().map(|str| Digit::parse(str)).collect();
        let mut map: HashMap<&str, u64> = HashMap::new();
    
        // The input with length 2 is number 1.
        let one = inputs.iter().find(|digit| digit.string.len() == 2).unwrap();
        map.insert(one.string.as_str(), 1);
    
        // The input with length 3 is number 7.
        let seven = inputs.iter().find(|digit| digit.string.len() == 3).unwrap();
        map.insert(seven.string.as_str(), 7);
    
        // The input with length 4 is number 4.
        let four = inputs.iter().find(|digit| digit.string.len() == 4).unwrap();
        map.insert(four.string.as_str(), 4);
    
        // The input with length 7 is number 8.
        let eight = inputs.iter().find(|digit| digit.string.len() == 7).unwrap();
        map.insert(eight.string.as_str(), 8);
    
        // Three digits have length 5: 2, 3 and 5.
        let mut len_five: Vec<&Digit> = inputs.iter().filter(|digit| digit.string.len() == 5).collect();
    
        // The one whose segments are a superset of 1 is 3.
        let three = *len_five.iter().find(|digit| digit.set.is_superset(&one.set)).unwrap();
        map.insert(three.string.as_str(), 3);
        len_five.retain(|&digit| digit as *const _ != three as *const _);
    
        // The one with three segments in common with 4 is 5.
        let five = *len_five.iter().find(|digit| digit.set.intersection(&four.set).count() == 3).unwrap();
        map.insert(five.string.as_str(), 5);
        len_five.retain(|&digit| digit as *const _ != five as *const _);
    
        // The remaining one is 2.
        let two = len_five[0];
        map.insert(two.string.as_str(), 2);
    
        // Three digits have length 6: 0, 6 and 9.
        let mut len_six: Vec<&Digit> = inputs.iter().filter(|digit| digit.string.len() == 6).collect();
    
        // The one that is *not* a superset of 5 is 0.
        let zero = *len_six.iter().find(|digit| !digit.set.is_superset(&five.set)).unwrap();
        map.insert(zero.string.as_str(), 0);
        len_six.retain(|&digit| digit as *const _ != zero as *const _);
    
        // The remaining one that is a superset of 7 is 9.
        let nine = *len_six.iter().find(|digit| digit.set.is_superset(&seven.set)).unwrap();
        map.insert(nine.string.as_str(), 9);
        len_six.retain(|&digit| digit as *const _ != nine as *const _);
    
        // The remaining one is 6.
        let six = len_six[0];
        map.insert(six.string.as_str(), 6);
    
        let digits: Vec<u64> = sections
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|str| str.chars().sorted().collect::<String>())
            .map(|str| *map.get(str.as_str()).unwrap())
            .collect();
        let mut value = 0u64;
        for digit in digits.iter() {
            value *= 10;
            value += digit;
        }
        Self {
            digits,
            value
        }
    }
}
