use std::convert::TryInto;

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<usize> {
    input_lines.map(|line| line.parse().unwrap()).collect()
}

fn find_first_invalid(xmas_numbers: &Vec<usize>) -> Option<(usize, usize)> {
    xmas_numbers
        .windows(26)
        .filter(|window| window.len() == 26)
        .enumerate()
        .find_map(|(i, window)| {
            let prev_25: [usize; 25] = window[..25].try_into().unwrap();
            let current = window[25];

            for (j, a) in prev_25[..prev_25.len() - 1].iter().enumerate() {
                for b in prev_25[(j + 1)..].iter() {
                    if a + b == current {
                        return None;
                    }
                }
            }
            Some((i + 25, current))
        })
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let xmas_numbers = parse_input(input_lines);

    let (_first_match_i, first_match) = find_first_invalid(&xmas_numbers).unwrap();

    first_match.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let xmas_numbers = parse_input(input_lines);

    let (_first_match_i, first_match) = find_first_invalid(&xmas_numbers).unwrap();

    let mut solution = None;

    for (i, a) in xmas_numbers[..(xmas_numbers.len() - 1)].iter().enumerate() {
        let mut subset_sum = *a;
        let mut min = *a;
        let mut max = *a;

        for b in xmas_numbers[(i + 1)..].iter() {
            subset_sum += b;
            if *b < min {
                min = *b;
            }
            if *b > max {
                max = *b;
            }

            if subset_sum == first_match {
                solution = Some(min + max);
                break;
            }
        }
    }

    solution.unwrap().to_string()
}
