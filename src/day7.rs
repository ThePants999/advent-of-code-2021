pub fn day7(input_lines: &[String]) -> (u64, u64) {
    let mut positions = input_lines[0].split(',').map(|pos_str| pos_str.parse::<i64>().expect("Non-numeric input")).collect::<Vec<_>>();
    positions.sort_unstable();

    // I know this isn't quite right as a median calculation. Sue me, it worked.
    let median = positions[(positions.len() / 2)];
    let part1 = determine_fuel_simple(&positions, median);

    let mean = positions.iter().sum::<i64>() / positions.len() as i64;
    let part2 = std::cmp::min(determine_fuel_complex(&positions, mean), determine_fuel_complex(&positions, mean + 1));

    (part1,part2)
}

fn determine_fuel_simple(positions: &[i64], target: i64) -> u64 {
    positions.iter().map(|pos| (*pos - target).abs()).sum::<i64>() as u64
}

fn determine_fuel_complex(positions: &[i64], target: i64) -> u64 {
    positions.iter().map(|pos| {
        let diff = (*pos - target).abs();
        (diff * (diff + 1)) / 2
    }).sum::<i64>() as u64
}