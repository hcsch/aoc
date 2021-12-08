use std::collections::HashMap;

use itertools::Itertools;

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> Vec<(Vec<[bool; 7]>, [[bool; 7]; 4])> {
    input_lines
        .map(|line| {
            let (signal_patterns_str, output_value_patterns_str) = line
                .split_once(" | ")
                .expect("expected separator between signal patterns and output values");

            let signal_patterns = signal_patterns_str
                .split(' ')
                .map(|pattern_str| {
                    let mut pattern = [false; 7];
                    for b in pattern_str.bytes() {
                        pattern[(b - b'a') as usize] = true;
                    }
                    pattern
                })
                .collect();

            let mut output_value_patterns = [[false; 7]; 4];
            for (output_value_pattern, pattern_str) in output_value_patterns
                .iter_mut()
                .zip(output_value_patterns_str.split(' '))
            {
                for b in pattern_str.bytes() {
                    output_value_pattern[(b - b'a') as usize] = true;
                }
            }

            (signal_patterns, output_value_patterns)
        })
        .collect()
}

fn permute_pattern(pattern: [bool; 7], permutation: [usize; 7]) -> [bool; 7] {
    let mut permuted_pattern = [false; 7];

    for (bit, target_index) in pattern.iter().copied().zip(permutation) {
        permuted_pattern[target_index] = bit;
    }

    permuted_pattern
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let entries = parse_input(input_lines);

    let solution: usize = entries
        .iter()
        .map(|(_signal_patterns, output_value_patterns)| {
            output_value_patterns
                .iter()
                .map(|pattern| pattern.iter().filter(|&&b| b).count())
                .filter(|num_set| [2, 3, 4, 7].contains(num_set))
                .count()
        })
        .sum();

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let entries = parse_input(input_lines);

    let pattern_to_digit_map: HashMap<[bool; 7], u8> = [
        ([true, true, true, false, true, true, true], 0),
        ([false, false, true, false, false, true, false], 1),
        ([true, false, true, true, true, false, true], 2),
        ([true, false, true, true, false, true, true], 3),
        ([false, true, true, true, false, true, false], 4),
        ([true, true, false, true, false, true, true], 5),
        ([true, true, false, true, true, true, true], 6),
        ([true, false, true, false, false, true, false], 7),
        ([true, true, true, true, true, true, true], 8),
        ([true, true, true, true, false, true, true], 9),
    ]
    .iter()
    .copied()
    .collect();

    let solution = entries
        .iter()
        .map(|(signal_patterns, output_value_patterns)| {
            let permutation: [usize; 7] = (0..7)
                .permutations(7)
                .map(|permutation| <[usize; 7]>::try_from(permutation.as_slice()).unwrap())
                .filter(|&permutation| {
                    signal_patterns
                        .iter()
                        .copied()
                        .map(|pattern| permute_pattern(pattern, permutation))
                        .filter_map(|pattern| pattern_to_digit_map.get(&pattern))
                        .count()
                        == 10
                })
                .next()
                .unwrap();

            output_value_patterns
                .iter()
                .copied()
                .map(|permuted_pattern| permute_pattern(permuted_pattern, permutation))
                .map(|pattern| *pattern_to_digit_map.get(&pattern).unwrap())
                .fold(0u16, |acc, digit| acc * 10 + digit as u16) as usize
        })
        .sum::<usize>();

    solution.to_string()
}
