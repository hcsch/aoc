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
    let horizontal_crab_positions = parse_input(input_lines);

    let (min, max) = horizontal_crab_positions
        .iter()
        .copied()
        .fold((0, u16::MAX), |(min, max), pos| {
            (min.min(pos), max.max(pos))
        });

    let (_optimal_target_pos, optimal_fuel_cost) = (min..=max)
        .map(|target_pos| {
            (
                target_pos,
                horizontal_crab_positions
                    .iter()
                    .map(|pos| pos.abs_diff(target_pos) as u64)
                    .sum::<u64>(),
            )
        })
        .min_by_key(|&(_target_pos, cost)| cost)
        .expect("no crab positions were given");

    optimal_fuel_cost.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let horizontal_crab_positions = parse_input(input_lines);

    let (min, max) = horizontal_crab_positions
        .iter()
        .copied()
        .fold((0, u16::MAX), |(min, max), pos| {
            (min.min(pos), max.max(pos))
        });

    let (_optimal_target_pos, optimal_fuel_cost) = (min..=max)
        .map(|target_pos| {
            (
                target_pos,
                horizontal_crab_positions
                    .iter()
                    .map(|pos| triangular_num(pos.abs_diff(target_pos) as u64))
                    .sum::<u64>(),
            )
        })
        .min_by_key(|&(_target_pos, cost)| cost)
        .expect("no crab positions were given");

    optimal_fuel_cost.to_string()
}
