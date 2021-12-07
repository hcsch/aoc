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

fn triangular_num(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let mut horizontal_crab_positions = parse_input(input_lines);

    horizontal_crab_positions.sort_unstable();

    // The median minimizes the sum absolute deviations.
    let optimal_target_pos = horizontal_crab_positions[horizontal_crab_positions.len() / 2];

    let optimal_fuel_cost = horizontal_crab_positions
        .iter()
        .map(|pos| pos.abs_diff(optimal_target_pos) as u64)
        .sum::<u64>();

    optimal_fuel_cost.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let horizontal_crab_positions = parse_input(input_lines);

    // The mean minimizes the sum of squared deviations, and apparently also
    // the sum of the triangular numbers of the deviations?
    // (which is what this puzzle requires).
    let optimal_target_pos = (horizontal_crab_positions
        .iter()
        .map(|&pos| pos as u64)
        .sum::<u64>()
        / horizontal_crab_positions.len() as u64) as u16;

    let optimal_fuel_cost = horizontal_crab_positions
        .iter()
        .map(|pos| triangular_num(pos.abs_diff(optimal_target_pos) as u64))
        .sum::<u64>();

    optimal_fuel_cost.to_string()
}
