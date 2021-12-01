use itertools::Itertools;

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let depths: Vec<u64> = input_lines.iter().map(|line| line.parse::<u64>().expect("Failed to parse input")).collect();
    let part1 = depths.iter().tuple_windows().filter(|(first, second)| second > first).count() as u64;
    let part2 = depths.iter().tuple_windows::<(_, _, _)>().map(|(first, second, third)| first + second + third).tuple_windows().filter(|(first, second)| second > first).count() as u64;
    (part1, part2) 
}