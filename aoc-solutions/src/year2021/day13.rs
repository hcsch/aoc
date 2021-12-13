#[derive(Clone, Copy)]
enum FoldingInstruction {
    AlongX(u16),
    AlongY(u16),
}

fn parse_input<I: Iterator<Item = String>>(
    mut input_lines: I,
) -> (Vec<(u16, u16)>, Vec<FoldingInstruction>) {
    let dots = input_lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            (x_str.parse().unwrap(), y_str.parse().unwrap())
        })
        .collect();
    let folding_instructions = input_lines
        .map(|line| {
            let (axis_str, pos_str) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            match axis_str {
                "x" => FoldingInstruction::AlongX(pos_str.parse().unwrap()),
                "y" => FoldingInstruction::AlongY(pos_str.parse().unwrap()),
                _ => panic!("expected a valid folding instruction line"),
            }
        })
        .collect();

    (dots, folding_instructions)
}

fn fold(dots: &mut Vec<(u16, u16)>, folding_instruction: FoldingInstruction) {
    for (x, y) in dots {
        match folding_instruction {
            FoldingInstruction::AlongX(fold_x) if *x >= fold_x => *x = 2 * fold_x - *x,
            FoldingInstruction::AlongY(fold_y) if *y >= fold_y => *y = 2 * fold_y - *y,
            _ => (),
        }
    }
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (mut dots, folding_instructions) = parse_input(input_lines);

    fold(&mut dots, folding_instructions[0]);

    dots.sort_unstable();
    dots.dedup();

    dots.len().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (mut dots, folding_instructions) = parse_input(input_lines);

    for folding_instruction in folding_instructions {
        fold(&mut dots, folding_instruction);
    }

    let (max_x, max_y) = dots.iter().fold((0, 0), |(max_x, max_y), &(dot_x, dot_y)| {
        (max_x.max(dot_x as usize), max_y.max(dot_y as usize))
    });

    let width = max_x + 1;
    let height = max_y + 1;

    let mut paper = vec![false; width * height];
    for (dot_x, dot_y) in dots {
        paper[dot_x as usize + dot_y as usize * width] = true;
    }

    let pattern: String = std::iter::once("\n")
        .chain(paper.chunks(width).flat_map(|row| {
            row.into_iter()
                .map(|on| if *on { "#" } else { "." })
                .chain(["\n"])
        }))
        .collect();

    pattern
}
