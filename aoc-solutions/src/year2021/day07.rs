fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> Vec<u16> {
    let crab_positions_line = input_lines
        .next()
        .expect("expected one line of comma-separated horizontal positions for the crabs");

    crab_positions_line
        .split(',')
        .map(|horizontal_position_str| {
            horizontal_position_str
                .parse::<u16>()
                .expect("expected comma separated unsigned integers")
        })
        .collect()
}

/// Computes the median, assuming that it is an integral number.
fn integer_median(nums: &mut Vec<u16>) -> u16 {
    let len = nums.len();
    if len % 2 != 0 {
        *nums.select_nth_unstable(len / 2).1
    } else {
        (*nums.select_nth_unstable(len / 2 - 1).1 + *nums.select_nth_unstable(len / 2).1) / 2
    }
}

fn triangular_num(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let mut horizontal_crab_positions = parse_input(input_lines);

    // The median minimizes the sum absolute deviations.
    let optimal_target_pos = integer_median(&mut horizontal_crab_positions);

    let optimal_fuel_cost = horizontal_crab_positions
        .iter()
        .map(|pos| pos.abs_diff(optimal_target_pos) as u32)
        .sum::<u32>();

    optimal_fuel_cost.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let horizontal_crab_positions = parse_input(input_lines);

    // The mean minimizes the sum of squared deviations, and apparently also
    // the sum of the triangular numbers of the deviations?
    // (which is what this puzzle requires).
    let optimal_target_pos = (horizontal_crab_positions
        .iter()
        .map(|&pos| pos as u32)
        .sum::<u32>()
        / horizontal_crab_positions.len() as u32) as u16;

    let optimal_fuel_cost = horizontal_crab_positions
        .iter()
        .map(|pos| triangular_num(pos.abs_diff(optimal_target_pos) as u32))
        .sum::<u32>();

    optimal_fuel_cost.to_string()
}
