use itertools::Itertools;

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let depths: Vec<u32> = input_lines
        .map(|l| {
            l.parse()
                .expect("Input was not solely lines of unsigned integers")
        })
        .collect();

    let solution = depths
        .iter()
        .copied()
        .tuple_windows()
        .map(|(prev_depth, cur_depth)| i64::from(cur_depth) - i64::from(prev_depth))
        .filter(|&depth_change| depth_change > 0)
        .count();

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let depths: Vec<u32> = input_lines
        .map(|l| {
            l.parse()
                .expect("Input was not solely lines of unsigned integers")
        })
        .collect();

    let solution = depths
        .iter()
        .copied()
        .tuple_windows()
        .map(|(depth0, depth1, depth2)| depth0 + depth1 + depth2)
        .tuple_windows()
        .map(|(prev_3depth_sum, cur_3depth_sum)| {
            i64::from(cur_3depth_sum) - i64::from(prev_3depth_sum)
        })
        .filter(|&depth_sum_change| depth_sum_change > 0)
        .count();

    solution.to_string()
}
