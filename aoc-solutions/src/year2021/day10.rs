#[derive(Clone, Copy, PartialEq, Eq)]
enum BracketPosition {
    Opening,
    Closing,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BracketKind {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketKind {
    pub fn illegal_closing_score(self) -> usize {
        match self {
            BracketKind::Round => 3,
            BracketKind::Square => 57,
            BracketKind::Curly => 1197,
            BracketKind::Angle => 25137,
        }
    }

    pub fn completion_score(self) -> usize {
        match self {
            BracketKind::Round => 1,
            BracketKind::Square => 2,
            BracketKind::Curly => 3,
            BracketKind::Angle => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bracket {
    kind: BracketKind,
    position: BracketPosition,
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<Vec<Bracket>> {
    input_lines
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'(' => Bracket {
                        kind: BracketKind::Round,
                        position: BracketPosition::Opening,
                    },
                    b')' => Bracket {
                        kind: BracketKind::Round,
                        position: BracketPosition::Closing,
                    },
                    b'[' => Bracket {
                        kind: BracketKind::Square,
                        position: BracketPosition::Opening,
                    },
                    b']' => Bracket {
                        kind: BracketKind::Square,
                        position: BracketPosition::Closing,
                    },
                    b'{' => Bracket {
                        kind: BracketKind::Curly,
                        position: BracketPosition::Opening,
                    },
                    b'}' => Bracket {
                        kind: BracketKind::Curly,
                        position: BracketPosition::Closing,
                    },
                    b'<' => Bracket {
                        kind: BracketKind::Angle,
                        position: BracketPosition::Opening,
                    },
                    b'>' => Bracket {
                        kind: BracketKind::Angle,
                        position: BracketPosition::Closing,
                    },
                    _ => panic!("expected only lines of brackets in input"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bracket_lines = parse_input(input_lines);

    let solution: usize = bracket_lines
        .iter()
        .filter_map(|brackets| {
            let mut opened_brackets_stack = Vec::new();
            for bracket in brackets {
                match bracket {
                    Bracket {
                        position: BracketPosition::Opening,
                        kind,
                    } => opened_brackets_stack.push(kind),
                    Bracket {
                        position: BracketPosition::Closing,
                        kind,
                    } => {
                        if let Some(innermost_open_bracket) = opened_brackets_stack.pop() {
                            if kind != innermost_open_bracket {
                                return Some(kind.illegal_closing_score());
                            }
                            // Brackets match, do nothing.
                        } else {
                            // Incomplete line, too many closing brackets.
                            return None;
                        }
                    }
                }
            }
            // Incomplete line, too many opening brackets,
            // or correct line.
            None
        })
        .sum();

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bracket_lines = parse_input(input_lines);

    let mut completion_scores: Vec<usize> = bracket_lines
        .iter()
        .filter_map(|brackets| {
            let mut opened_brackets_stack = Vec::new();
            for bracket in brackets {
                match bracket {
                    Bracket {
                        position: BracketPosition::Opening,
                        kind,
                    } => opened_brackets_stack.push(kind),
                    Bracket {
                        position: BracketPosition::Closing,
                        kind,
                    } => {
                        if let Some(innermost_open_bracket) = opened_brackets_stack.pop() {
                            if kind != innermost_open_bracket {
                                return None;
                            }
                        } else {
                            // Incomplete line, too many closing brackets.
                            unreachable!(
                                "input guaranteed not to contain unmatched closing brackets"
                            );
                        }
                    }
                }
            }
            if !opened_brackets_stack.is_empty() {
                Some(
                    opened_brackets_stack
                        .iter()
                        .rev()
                        .copied()
                        .fold(0, |score, kind| score * 5 + kind.completion_score()),
                )
            } else {
                // Correctly
                unreachable!("input guaranteed not to contain correctly bracketed lines")
            }
            // Incomplete line, too many opening brackets,
            // or correct line.
        })
        .collect();

    completion_scores.sort_unstable();

    completion_scores[completion_scores.len() / 2].to_string()
}
