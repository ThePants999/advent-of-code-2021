use std::collections::HashMap;

pub fn day10(input_lines: &[String]) -> (u64, u64) {
    let illegal_scores: HashMap<char, u64> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut illegal_score = 0u64;
    let mut incomplete_scores: Vec<u64> = Vec::with_capacity(input_lines.len());
    for line in input_lines {
        let mut chunks: Vec<char> = Vec::with_capacity(line.len() / 2);
        let mut legal = true;
        for c in line.chars() {
            let current_chunk = chunks.last();
            match c {
                '(' => chunks.push(')'),
                '[' => chunks.push(']'),
                '{' => chunks.push('}'),
                '<' => chunks.push('>'),
                ')' | ']' | '}' | '>' => {
                    if current_chunk == Some(&c) {
                        chunks.pop();
                    } else {
                        illegal_score += illegal_scores.get(&c).unwrap();
                        legal = false;
                        break;
                    }
                },
                _ => unreachable!("Invalid character in input"),
            }
        }

        if legal && !chunks.is_empty() {
            let mut incomplete_score = 0u64;
            while let Some(chunk) = chunks.pop() {
                incomplete_score *= 5;
                incomplete_score += match chunk {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }
            incomplete_scores.push(incomplete_score);
        }
    }
    let part1 = illegal_score;
    incomplete_scores.sort_unstable();
    let part2 = incomplete_scores[incomplete_scores.len() / 2];

    (part1,part2)
}