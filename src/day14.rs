use std::collections::HashMap;
use itertools::Itertools;

use crate::utils;

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    let input_groups = utils::group_lines_split_by_empty_line(input_lines);

    let final_char = input_groups[0][0].chars().last().unwrap();
    let mut polymer: HashMap<(char, char), u64> = HashMap::with_capacity(input_groups[1].len());
    for tuple in input_groups[0][0].chars().tuple_windows() {
        let count = polymer.entry(tuple).or_insert(0);
        *count += 1;
    }
    let rules = input_groups[1].iter().map(|line| Rule::parse(line)).collect::<Vec<_>>();

    for _ in 0..10 {
        polymer = apply_step(polymer, &rules);
    }
    let part1 = calc_quantity(&polymer, &final_char);

    for _ in 0..30 {
        polymer = apply_step(polymer, &rules);
    }
    let part2 = calc_quantity(&polymer, &final_char);

    (part1,part2)
}

fn apply_step(polymer: HashMap<(char, char), u64>, rules: &[Rule]) -> HashMap<(char, char), u64> {
    let mut new_polymer: HashMap<(char, char), u64> = HashMap::with_capacity(polymer.len());

    for rule in rules {
        let count = polymer.get(&rule.pair).unwrap_or(&0);
        let new_count = new_polymer.entry(rule.result1).or_insert(0);
        *new_count += count;
        let new_count = new_polymer.entry(rule.result2).or_insert(0);
        *new_count += count;
    }

    new_polymer
}

fn calc_quantity(polymer: &HashMap<(char, char), u64>, final_char: &char) -> u64 {
    let mut char_count: HashMap<char, u64> = HashMap::with_capacity(26);
    for (c, new_count) in polymer.iter().map(|((c, _), count)| (c, count)) {
        let count = char_count.entry(*c).or_insert(0);
        *count += new_count;
    }
    *char_count.entry(*final_char).or_insert(0) += 1;
    char_count.values().max().unwrap() - char_count.values().min().unwrap()
}

struct Rule {
    pair: (char, char),
    result1: (char, char),
    result2: (char, char),
}

impl Rule {
    fn parse(input_line: &str) -> Self {
        let tokens: Vec<&str> = input_line.split_ascii_whitespace().collect();
        let pair1 = tokens[0].chars().next().unwrap();
        let pair2 = tokens[0].chars().nth(1).unwrap();
        let insertion = tokens[2].chars().next().unwrap();
        Self {
            pair: (pair1, pair2),
            result1: (pair1, insertion),
            result2: (insertion, pair2),
        }
    }
}
