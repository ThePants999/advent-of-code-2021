pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let num_bits = input_lines[0].trim().len();
    let full_mask = u64::from_str_radix("1".repeat(num_bits).as_str(), 2).unwrap();
    let numbers: Vec<u64> = input_lines.iter().map(|line| u64::from_str_radix(line, 2).expect("Input line not binary")).collect();

    let mut gamma = 0u64;
    let mut oxygen_generator_numbers = numbers.clone();
    let mut co2_scrubber_numbers = numbers.clone();

    for index in 0..num_bits {
        let mask = 1 << (num_bits - index - 1);

        // Part 1
        let (bit_set, bit_not_set) = split_by_bit_set(&numbers, mask);
        if bit_set.len() >= bit_not_set.len() {
            gamma |= mask;
        }

        // Part 2
        if oxygen_generator_numbers.len() > 1 {
            let (o2_set, o2_not_set) = split_by_bit_set(&oxygen_generator_numbers, mask);
            oxygen_generator_numbers = if o2_set.len() >= o2_not_set.len() { o2_set } else { o2_not_set };
        }
        if co2_scrubber_numbers.len() > 1 {
            let (co2_set, co2_not_set) = split_by_bit_set(&co2_scrubber_numbers, mask);
            co2_scrubber_numbers = if co2_set.len() < co2_not_set.len() { co2_set } else { co2_not_set };
        }
    }
    let epsilon = !gamma & full_mask;

    let part1 = gamma * epsilon;
    let part2 = oxygen_generator_numbers[0] * co2_scrubber_numbers[0];
    (part1, part2)
}

fn split_by_bit_set(values: &[u64], mask: u64) -> (Vec<u64>, Vec<u64>) {
    values.iter().partition(|&value| (value & mask) > 0)
}
