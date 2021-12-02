fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<Vec<bool>> {
    input_lines
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| match c {
                    b'#' => true,
                    b'.' => false,
                    _ => panic!("Encountered unexpected character in input"),
                })
                .collect()
        })
        .collect()
}

fn count_trees_encountered(tree_map: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    let (dx, dy) = slope;
    tree_map
        .iter()
        .step_by(dy)
        .zip((0..).step_by(dx))
        .filter_map(|(row, x)| row[x % row.len()].then_some(()))
        .count()
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let tree_map = parse_input(input_lines);
    count_trees_encountered(&tree_map, (3, 1))
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let tree_map = parse_input(input_lines);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| count_trees_encountered(&tree_map, *slope))
        .product()
}
