pub fn day6(input_lines: &[String]) -> (u64, u64) {
    let mut fish_by_time: [u64; 9] = [0; 9];
    input_lines[0].split(',').map(|time| time.parse::<usize>().expect("Invalid input")).for_each(|time| fish_by_time[time] += 1);

    for _ in 0..80 {
        simulate_day(&mut fish_by_time);
    }
    let part1 = fish_by_time.iter().sum();

    for _ in 80..256 {
        simulate_day(&mut fish_by_time);
    }
    let part2 = fish_by_time.iter().sum();

    (part1, part2)
}

fn simulate_day(fish_by_time: &mut [u64; 9]) {
    let spawning_fish = fish_by_time[0];
    for time in 0..8 {
        fish_by_time[time] = fish_by_time[time + 1];
    }
    fish_by_time[8] = spawning_fish;
    fish_by_time[6] += spawning_fish;
}