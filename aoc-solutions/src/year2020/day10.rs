fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<usize> {
    input_lines.map(|line| line.parse().unwrap()).collect()
}

pub fn compute_joltage_deltas<'a>(
    charger_ratings: &'a Vec<usize>,
) -> impl Iterator<Item = usize> + 'a {
    charger_ratings
        .iter()
        .zip(charger_ratings[1..].iter())
        .map(|(a, b)| b - a)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> i32 {
    let mut charger_ratings = parse_input(input_lines);

    charger_ratings.push(0);
    charger_ratings.push(charger_ratings.iter().max().unwrap() + 3);

    charger_ratings.sort();

    let (num_ones, num_threes) = compute_joltage_deltas(&charger_ratings).fold(
        (0, 0),
        |(num_ones, num_threes), dj| match dj {
            1 => (num_ones + 1, num_threes),
            3 => (num_ones, num_threes + 1),
            _ => (num_ones, num_threes),
        },
    );

    num_ones * num_threes
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let mut charger_ratings = parse_input(input_lines);

    charger_ratings.push(0);
    charger_ratings.push(charger_ratings.iter().max().unwrap() + 3);

    charger_ratings.sort();

    let mut num_cfgs = [1, 0, 0];
    for dj in compute_joltage_deltas(&charger_ratings) {
        match dj {
            1 => {
                num_cfgs = [
                    num_cfgs[0] + num_cfgs[1] + num_cfgs[2],
                    num_cfgs[0],
                    num_cfgs[1],
                ]
            }
            2 => num_cfgs = [num_cfgs[0] + num_cfgs[1], 0, num_cfgs[0]],
            3 => num_cfgs = [num_cfgs[0], 0, 0],
            _ => unreachable!("Unexpected joltage difference > 3 or < 1"),
        }
    }

    num_cfgs.iter().sum()
}
