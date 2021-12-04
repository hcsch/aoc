use std::cmp;

struct BingoBoard {
    numbers: [u8; 5 * 5],
    marked: [bool; 5 * 5],
}

impl BingoBoard {
    pub fn has_won(&self) -> bool {
        (0..5)
            .map(|i| {
                let num_marked_in_row: u8 = self
                    .marked
                    .iter()
                    .copied()
                    .skip(i * 5)
                    .take(5)
                    .map(|marked| marked as u8)
                    .sum();
                let num_marked_in_col: u8 = self
                    .marked
                    .iter()
                    .copied()
                    .skip(i)
                    .step_by(5)
                    .map(|marked| marked as u8)
                    .sum();

                cmp::max(num_marked_in_row, num_marked_in_col)
            })
            .max()
            .unwrap()
            >= 5
    }

    pub fn score(&self, last_called_num: u8) -> usize {
        self.numbers
            .iter()
            .copied()
            .zip(self.marked)
            .filter_map(|(num, marked)| (!marked).then_some(num))
            .map(|unmarked_num| unmarked_num as usize)
            .sum::<usize>()
            * last_called_num as usize
    }

    pub fn mark_if_present(&mut self, called_num: u8) {
        self.numbers
            .iter()
            .copied()
            .zip(self.marked.as_mut())
            .filter(|(num, _marked)| *num == called_num)
            .for_each(|(_num, marked)| *marked = true)
    }
}

fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> (Vec<u8>, Vec<BingoBoard>) {
    let called_nums = input_lines
        .next()
        .expect("expected line with numbers, found EOF")
        .split(',')
        .map(|num| num.parse().expect("non-number on first line"))
        .collect();

    let mut bingo_boards = vec![];

    let mut board_nums = vec![];

    loop {
        // Drop empty line between boards or stop if at end of file
        if input_lines.next().is_none() {
            break;
        }

        for _ in 0..5 {
            board_nums.extend(
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

        let board_nums_array = board_nums
            .as_slice()
            .try_into()
            .expect("incorrect number of numbers per board, expected 5 * 5 = 25");

        bingo_boards.push(BingoBoard {
            numbers: board_nums_array,
            marked: [false; 5 * 5],
        });

        board_nums.clear();
    }

    (called_nums, bingo_boards)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (called_nums, mut bingo_boards) = parse_input(input_lines);

    let mut solution = None;

    for called_num in called_nums {
        for board in bingo_boards.iter_mut() {
            board.mark_if_present(called_num);
        }

        if let Some(board) = bingo_boards.iter().find(|board| board.has_won()) {
            solution = Some(board.score(called_num));
            break;
        }
    }

    solution.unwrap().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (called_nums, mut bingo_boards) = parse_input(input_lines);

    let mut solution = None;

    for called_num in called_nums {
        for board in bingo_boards.iter_mut() {
            board.mark_if_present(called_num);
        }

        if bingo_boards.len() > 1 {
            bingo_boards.retain(|board| !board.has_won())
        } else {
            if bingo_boards[0].has_won() {
                solution = Some(bingo_boards[0].score(called_num));
                break;
            }
        }
    }

    solution.unwrap().to_string()
}
