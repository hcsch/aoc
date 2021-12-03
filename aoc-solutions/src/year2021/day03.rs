pub fn num_to_bit_array(num: u16) -> [u16; 16] {
    [
        ((num >> 15) & 1),
        ((num >> 14) & 1),
        ((num >> 13) & 1),
        ((num >> 12) & 1),
        ((num >> 11) & 1),
        ((num >> 10) & 1),
        ((num >> 9) & 1),
        ((num >> 8) & 1),
        ((num >> 7) & 1),
        ((num >> 6) & 1),
        ((num >> 5) & 1),
        ((num >> 4) & 1),
        ((num >> 3) & 1),
        ((num >> 2) & 1),
        ((num >> 1) & 1),
        ((num >> 0) & 1),
    ]
}

pub fn sum_arrays(mut a: [u16; 16], b: [u16; 16]) -> [u16; 16] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a = *a + b);
    a
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let diagnostic_nums: Vec<u16> = input_lines
        .map(|l| u16::from_str_radix(&l, 2).expect("Input was not solely lines of binary integers"))
        .collect();

    let ones_count_per_bit = diagnostic_nums
        .iter()
        .copied()
        .map(|diagnostic_num| num_to_bit_array(diagnostic_num))
        .fold([0; 16], |ones_count_per_bit, bit_array| {
            sum_arrays(ones_count_per_bit, bit_array)
        });

    let mut gamma_rate = 0u16;
    let mut epsilon_rate = 0u16;

    let first_bit_i = ones_count_per_bit
        .iter()
        .copied()
        .enumerate()
        .find_map(|(i, count)| if count > 0 { Some(i) } else { None })
        .unwrap_or(15);

    let total_num_diagnositc_nums = diagnostic_nums.len();
    for (i, ones_count) in ones_count_per_bit.iter().copied().enumerate() {
        gamma_rate |= if ones_count as usize >= total_num_diagnositc_nums / 2 {
            1 << (15 - i)
        } else {
            0
        };
        epsilon_rate |= if i >= first_bit_i && (ones_count as usize) < total_num_diagnositc_nums / 2
        {
            1 << (15 - i)
        } else {
            0
        };
    }

    let solution = gamma_rate as u32 * epsilon_rate as u32;

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let diagnostic_nums: Vec<u16> = input_lines
        .map(|l| u16::from_str_radix(&l, 2).expect("Input was not solely lines of binary integers"))
        .collect();

    let ones_count_per_bit = diagnostic_nums
        .iter()
        .copied()
        .map(|diagnostic_num| num_to_bit_array(diagnostic_num))
        .fold([0; 16], |ones_count_per_bit, bit_array| {
            sum_arrays(ones_count_per_bit, bit_array)
        });

    let first_bit_i = ones_count_per_bit
        .iter()
        .copied()
        .enumerate()
        .find_map(|(i, count)| if count > 0 { Some(i) } else { None })
        .unwrap_or(15);

    let mut nums_matching_most_common: Vec<(u16, [u16; 16])> = diagnostic_nums
        .iter()
        .copied()
        .map(|diagnostic_num| (diagnostic_num, num_to_bit_array(diagnostic_num)))
        .collect();

    let mut i = first_bit_i;
    while nums_matching_most_common.len() > 1 {
        let most_common_bit = (nums_matching_most_common
            .iter()
            .map(|(_, bits)| bits[i] as usize)
            .sum::<usize>()
            >= (nums_matching_most_common.len() + 1) / 2) as u16;

        nums_matching_most_common.retain_mut(|(_, bits)| bits[i] == most_common_bit);
        i += 1;
    }

    let oxygen_generator_rating = nums_matching_most_common.first().unwrap().0;

    let mut nums_matching_least_common: Vec<(u16, [u16; 16])> = diagnostic_nums
        .iter()
        .copied()
        .map(|diagnostic_num| (diagnostic_num, num_to_bit_array(diagnostic_num)))
        .collect();

    let mut i = first_bit_i;
    while nums_matching_least_common.len() > 1 {
        let num_ones = nums_matching_least_common
            .iter()
            .map(|(_, bits)| bits[i] as usize)
            .sum::<usize>();
        let least_common_bit = ((nums_matching_least_common.len() % 2 == 0
            && num_ones < nums_matching_least_common.len() / 2)
            || (nums_matching_least_common.len() % 2 != 0
                && num_ones <= nums_matching_least_common.len() / 2))
            as u16;

        nums_matching_least_common.retain_mut(|(_, bits)| bits[i] == least_common_bit);
        i += 1;
    }

    let co2_scrubber_rating = nums_matching_least_common.first().unwrap().0;

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
