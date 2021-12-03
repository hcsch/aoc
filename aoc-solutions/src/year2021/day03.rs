/// Returns an array with the counts of bits set over all numbers for each bit index.
/// Bit indices are LSB to MSB ascending.
pub fn count_set_per_index<I: Iterator<Item = u16>>(nums: I) -> [u16; 16] {
    nums.fold([0; 16], |mut acc, num| {
        for i in 0usize..16 {
            acc[i] += num >> i & 1;
        }
        acc
    })
}

pub fn count_set_at_index<I: Iterator<Item = u16>>(nums: I, i: usize) -> usize {
    nums.map(|num| usize::from(num >> i & 1)).sum()
}

pub fn num_significant_bits(count_set_per_index: [u16; 16]) -> usize {
    16 - count_set_per_index
        .iter()
        .rev()
        .copied()
        .enumerate()
        .find_map(|(i, count)| if count > 0 { Some(i) } else { None })
        .unwrap_or(16)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let diagnostic_nums: Vec<u16> = input_lines
        .map(|l| u16::from_str_radix(&l, 2).expect("Input was not solely lines of binary integers"))
        .collect();

    let count_set_per_index = count_set_per_index(diagnostic_nums.iter().copied());

    let mut gamma_rate = 0u16;
    let mut epsilon_rate = 0u16;

    let num_significant_bits = num_significant_bits(count_set_per_index);

    for (i, ones_count) in count_set_per_index
        .iter()
        .copied()
        .take(num_significant_bits)
        .enumerate()
    {
        let most_common_value = (ones_count as usize >= diagnostic_nums.len() / 2) as u16;
        gamma_rate |= most_common_value << i;
        epsilon_rate |= (1 - most_common_value) << i;
    }

    let solution = gamma_rate as u32 * epsilon_rate as u32;

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let diagnostic_nums: Vec<u16> = input_lines
        .map(|l| u16::from_str_radix(&l, 2).expect("Input was not solely lines of binary integers"))
        .collect();

    let count_set_per_index = count_set_per_index(diagnostic_nums.iter().copied());

    let num_significant_bits = num_significant_bits(count_set_per_index);

    let mut nums_matching_most_common: Vec<u16> = diagnostic_nums.clone();

    for i in (0..num_significant_bits).rev() {
        let num_set: usize = count_set_at_index(nums_matching_most_common.iter().copied(), i);
        let num_unset = nums_matching_most_common.len() - num_set;
        let most_common_value = if num_set >= num_unset { 1 } else { 0 };

        nums_matching_most_common.retain(|num| (num >> i & 1) == most_common_value);

        if nums_matching_most_common.len() <= 1 {
            break;
        }
    }

    let oxygen_generator_rating = *nums_matching_most_common.first().unwrap();

    let mut nums_matching_least_common: Vec<u16> = diagnostic_nums.clone();

    for i in (0..num_significant_bits).rev() {
        let num_set: usize = count_set_at_index(nums_matching_least_common.iter().copied(), i);
        let num_unset = nums_matching_least_common.len() - num_set;
        let least_common_value = if num_unset <= num_set { 0 } else { 1 };

        nums_matching_least_common.retain(|num| (num >> i & 1) == least_common_value);

        if nums_matching_least_common.len() <= 1 {
            break;
        }
    }

    let co2_scrubber_rating = *nums_matching_least_common.first().unwrap();

    let solution = oxygen_generator_rating as u32 * co2_scrubber_rating as u32;

    solution.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: [&'static str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_puzzle1_example_input() {
        assert_eq!(
            solve_puzzle1(EXAMPLE_INPUT.into_iter().map(|s| s.to_owned())),
            "198"
        );
    }

    #[test]
    fn test_puzzle2_example_input() {
        assert_eq!(
            solve_puzzle2(EXAMPLE_INPUT.into_iter().map(|s| s.to_owned())),
            "230"
        );
    }
}
