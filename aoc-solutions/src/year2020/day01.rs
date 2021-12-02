use std::convert::identity;

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> i64 {
    let expenses: Vec<i64> = input_lines
        .map(|l| {
            l.parse::<i64>()
                .expect("Input was not solely lines of integers")
        })
        .collect();

    let solution = expenses
        .iter()
        .enumerate()
        .map(|(i, e0)| {
            let mut maybe_solution = None;
            for e1 in expenses.iter().skip(i) {
                if e0 + e1 == 2020 {
                    maybe_solution = Some(e0 * e1);
                    break;
                }
            }
            maybe_solution
        })
        .find_map(identity)
        .expect("No two expenses add up to 2020");

    solution
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> i64 {
    let expenses: Vec<i64> = input_lines
        .map(|l| {
            l.parse::<i64>()
                .expect("Input was not solely lines of integers")
        })
        .collect();

    let solution = expenses
        .iter()
        .enumerate()
        .map(|(i, e0)| {
            let mut maybe_solution = None;
            'outer: for (j, e1) in expenses.iter().skip(i).enumerate() {
                for e2 in expenses.iter().skip(j) {
                    if e0 + e1 + e2 == 2020 {
                        maybe_solution = Some(e0 * e1 * e2);
                        break 'outer;
                    }
                }
            }
            maybe_solution
        })
        .find_map(identity)
        .expect("No three expenses add up to 2020");

    solution
}
