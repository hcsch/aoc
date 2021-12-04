use ndarray::{Array2, Axis};

struct BingoBoard {
    numbers: Array2<u8>,
    marked: Array2<bool>,
}

impl BingoBoard {
    pub fn has_won(&self) -> bool {
        *self
            .marked
            .map(|&num_marked| num_marked as u8)
            .sum_axis(Axis(0))
            .iter()
            .max()
            .unwrap()
            >= 5
            || *self
                .marked
                .map(|&num_marked| num_marked as u8)
                .sum_axis(Axis(1))
                .iter()
                .max()
                .unwrap()
                >= 5
    }

    pub fn get_score(&self, last_called_num: u8) -> usize {
        (&self.numbers * self.marked.map(|&num_marked| (!num_marked) as u8))
            .iter()
            .map(|&unmarked_num_or_zero| unmarked_num_or_zero as usize)
            .sum::<usize>()
            * last_called_num as usize
    }
}

fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> (Vec<u8>, Vec<BingoBoard>) {
    let called_numbers = input_lines
        .next()
        .expect("expected line with numbers, found EOF")
        .split(',')
        .map(|num| num.parse().expect("non-number on first line"))
        .collect();

    let mut bingo_boards = vec![];

    loop {
        // Drop empty line between boards or stop if at end of file
        if input_lines.next().is_none() {
            break;
        }

        let mut board_numbers = vec![];

        for _ in 0..5 {
            board_numbers.extend(
                input_lines
                    .next()
                    .expect("expected board line, found EOF")
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<u8>()
                            .expect("non-number where a bingo board was expected")
                    }),
            );
        }

        bingo_boards.push(BingoBoard {
            numbers: Array2::from_shape_vec((5, 5), board_numbers).unwrap(),
            marked: Array2::from_elem((5, 5), false),
        });
    }

    (called_numbers, bingo_boards)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (called_numbers, mut bingo_boards) = parse_input(input_lines);

    let mut solution = None;

    for called_num in called_numbers {
        for board in bingo_boards.iter_mut() {
            for (index, &num) in board.numbers.indexed_iter() {
                if num == called_num {
                    board.marked[index] = true;
                }
            }
        }

        if let Some(board) = bingo_boards.iter().find(|board| board.has_won()) {
            solution = Some(board.get_score(called_num));
            break;
        }
    }

    solution.unwrap().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (called_numbers, mut bingo_boards) = parse_input(input_lines);

    let mut solution = None;

    for called_num in called_numbers {
        for board in bingo_boards.iter_mut() {
            for (index, &num) in board.numbers.indexed_iter() {
                if num == called_num {
                    board.marked[index] = true;
                }
            }
        }

        if bingo_boards.len() > 1 {
            bingo_boards.retain(|board| !board.has_won())
        } else {
            if bingo_boards[0].has_won() {
                solution = Some(bingo_boards[0].get_score(called_num));
                break;
            }
        }
    }

    solution.unwrap().to_string()
}
