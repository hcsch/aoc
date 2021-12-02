use std::fmt::{self, Display};

use ndarray::Array2;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SeatState {
    Empty,
    Occupied,
    NotPresent, // No seat there, only floor
}

impl Display for SeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "L",
                Self::Occupied => "#",
                Self::NotPresent => ".",
            }
        )
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Array2<SeatState> {
    let mut num_columns = None;
    let seat_states_raw = input_lines
        .flat_map(|line| {
            let current_line_length = line.len();
            match num_columns.map(|nc| nc == current_line_length) {
                None => num_columns = Some(current_line_length),
                Some(true) => (),
                Some(false) => panic!("Not all lines are of the same length"),
            }
            line.as_bytes().to_owned().into_iter().map(|c| match c {
                b'L' => SeatState::Empty,
                b'#' => SeatState::Occupied,
                b'.' => SeatState::NotPresent,
                _ => panic!("Encountered unexpected character in input"),
            })
        })
        .collect::<Vec<_>>();

    let num_columns = num_columns.unwrap();

    Array2::from_shape_vec(
        (seat_states_raw.len() / num_columns, num_columns),
        seat_states_raw,
    )
    .unwrap()
}

#[inline]
fn checked_add(a: usize, b: isize) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub((-b) as usize)
    }
}

const NEIGHBORHOOD: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct StateUpdateIter<'a> {
    previous_state: &'a Array2<SeatState>,
    x: usize,
    y: usize,
}

impl<'a> StateUpdateIter<'a> {
    pub fn new(current_state: &'a Array2<SeatState>) -> Self {
        Self {
            previous_state: current_state,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for StateUpdateIter<'a> {
    type Item = SeatState;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        if self.y < self.previous_state.nrows() && self.x < self.previous_state.ncols() {
            let occupied_neighboring_seats = NEIGHBORHOOD
                .iter()
                .filter_map(|(dx, dy)| {
                    self.previous_state
                        .get((checked_add(self.y, *dy)?, checked_add(self.x, *dx)?))
                })
                .filter(|state| **state == SeatState::Occupied)
                .count();

            let old_state = self.previous_state[[self.y, self.x]];
            let new_state = match (old_state, occupied_neighboring_seats) {
                (SeatState::Empty, 0) => SeatState::Occupied,
                (SeatState::Occupied, 4..=8) => SeatState::Empty,
                (old_state, _) => old_state,
            };

            if self.x + 1 == self.previous_state.ncols() {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }

            Some(new_state)
        } else {
            None
        }
    }
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let mut seat_states = parse_input(input_lines);

    loop {
        let previous_states = seat_states.clone();
        seat_states
            .iter_mut()
            .zip(StateUpdateIter::new(&previous_states))
            .for_each(|(target, src)| *target = src);

        if seat_states == previous_states {
            break;
        }
    }

    seat_states
        .iter()
        .filter(|s| **s == SeatState::Occupied)
        .count()
}

struct TweakedStateUpdateIter<'a> {
    previous_state: &'a Array2<SeatState>,
    x: usize,
    y: usize,
}

impl<'a> TweakedStateUpdateIter<'a> {
    pub fn new(current_state: &'a Array2<SeatState>) -> Self {
        Self {
            previous_state: current_state,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for TweakedStateUpdateIter<'a> {
    type Item = SeatState;

    fn next(&mut self) -> std::option::Option<<Self as Iterator>::Item> {
        if self.y < self.previous_state.nrows() && self.x < self.previous_state.ncols() {
            let occupied_neighboring_seats = NEIGHBORHOOD
                .iter()
                .filter_map(|(dx, dy)| {
                    itertools::iterate((*dx, *dy), |(dx2, dy2)| (*dx2 + dx, *dy2 + dy))
                        .map(|(dx, dy)| {
                            self.previous_state
                                .get((checked_add(self.y, dy)?, checked_add(self.x, dx)?))
                        })
                        .find(|maybe_state| *maybe_state != Some(&SeatState::NotPresent))
                        .unwrap()
                })
                .filter(|state| **state == SeatState::Occupied)
                .count();

            let old_state = self.previous_state[[self.y, self.x]];
            let new_state = match (old_state, occupied_neighboring_seats) {
                (SeatState::Empty, 0) => SeatState::Occupied,
                (SeatState::Occupied, 5..=8) => SeatState::Empty,
                (old_state, _) => old_state,
            };

            if self.x + 1 == self.previous_state.ncols() {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }

            Some(new_state)
        } else {
            None
        }
    }
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let mut seat_states = parse_input(input_lines);

    loop {
        let previous_states = seat_states.clone();
        seat_states
            .iter_mut()
            .zip(TweakedStateUpdateIter::new(&previous_states))
            .for_each(|(target, src)| *target = src);

        if seat_states == previous_states {
            break;
        }
    }

    seat_states
        .iter()
        .filter(|s| **s == SeatState::Occupied)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::lazy::SyncLazy;

    use indoc::indoc;
    use itertools::Itertools;

    const STATE_STRS: [&str; 6] = [
        indoc! {"
                L.LL.LL.LL
                LLLLLLL.LL
                L.L.L..L..
                LLLL.LL.LL
                L.LL.LL.LL
                L.LLLLL.LL
                ..L.L.....
                LLLLLLLLLL
                L.LLLLLL.L
                L.LLLLL.LL
            "},
        indoc! {"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
            "},
        indoc! {"
                #.LL.L#.##
                #LLLLLL.L#
                L.L.L..L..
                #LLL.LL.L#
                #.LL.LL.LL
                #.LLLL#.##
                ..L.L.....
                #LLLLLLLL#
                #.LLLLLL.L
                #.#LLLL.##
            "},
        indoc! {"
                #.##.L#.##
                #L###LL.L#
                L.#.#..#..
                #L##.##.L#
                #.##.LL.LL
                #.###L#.##
                ..#.#.....
                #L######L#
                #.LL###L.L
                #.#L###.##
            "},
        indoc! {"
                #.#L.L#.##
                #LLL#LL.L#
                L.L.L..#..
                #LLL.##.L#
                #.LL.LL.LL
                #.LL#L#.##
                ..L.L.....
                #L#LLLL#L#
                #.LLLLLL.L
                #.#L#L#.##
            "},
        indoc! {"
                #.#L.L#.##
                #LLL#LL.L#
                L.#.L..#..
                #L##.##.L#
                #.#L.LL.LL
                #.#L#L#.##
                ..L.L.....
                #L#L##L#L#
                #.LLLLLL.L
                #.#L#L#.##
            "},
    ];

    static STATE_ARRAYS: SyncLazy<[Array2<SeatState>; 6]> = SyncLazy::new(|| {
        [
            parse_input(STATE_STRS[0].lines().map(|l| l.to_owned())),
            parse_input(STATE_STRS[1].lines().map(|l| l.to_owned())),
            parse_input(STATE_STRS[2].lines().map(|l| l.to_owned())),
            parse_input(STATE_STRS[3].lines().map(|l| l.to_owned())),
            parse_input(STATE_STRS[4].lines().map(|l| l.to_owned())),
            parse_input(STATE_STRS[5].lines().map(|l| l.to_owned())),
        ]
    });

    #[test]
    fn test_solve_puzzle1() {
        assert_eq!(
            solve_puzzle1(STATE_STRS[0].lines().map(|l| l.to_owned())),
            37
        );
    }

    fn state_array_to_string(state_array: &Array2<SeatState>) -> String {
        let mut state_array_string = String::new();
        state_array_string.push_str(
            &state_array
                .rows()
                .into_iter()
                .map(|r| r.iter().map(|s| s.to_string()).join(""))
                .join("\n"),
        );
        state_array_string.push('\n');
        state_array_string
    }

    #[test]
    fn test_state_update_iter0() {
        let mut current_state = STATE_ARRAYS[0].clone();
        current_state
            .iter_mut()
            .zip(StateUpdateIter::new(&STATE_ARRAYS[0]))
            .for_each(|(target, src)| *target = src);
        assert_eq!(state_array_to_string(&current_state), STATE_STRS[1]);
    }

    #[test]
    fn test_state_update_iter1() {
        let mut current_state = STATE_ARRAYS[1].clone();
        current_state
            .iter_mut()
            .zip(StateUpdateIter::new(&STATE_ARRAYS[1]))
            .for_each(|(target, src)| *target = src);
        assert_eq!(state_array_to_string(&current_state), STATE_STRS[2]);
    }

    #[test]
    fn test_state_update_iter2() {
        let mut current_state = STATE_ARRAYS[2].clone();
        current_state
            .iter_mut()
            .zip(StateUpdateIter::new(&STATE_ARRAYS[2]))
            .for_each(|(target, src)| *target = src);
        assert_eq!(state_array_to_string(&current_state), STATE_STRS[3]);
    }

    #[test]
    fn test_state_update_iter3() {
        let mut current_state = STATE_ARRAYS[3].clone();
        current_state
            .iter_mut()
            .zip(StateUpdateIter::new(&STATE_ARRAYS[3]))
            .for_each(|(target, src)| *target = src);
        assert_eq!(state_array_to_string(&current_state), STATE_STRS[4]);
    }

    #[test]
    fn test_state_update_iter4() {
        let mut current_state = STATE_ARRAYS[4].clone();
        current_state
            .iter_mut()
            .zip(StateUpdateIter::new(&STATE_ARRAYS[4]))
            .for_each(|(target, src)| *target = src);
        assert_eq!(state_array_to_string(&current_state), STATE_STRS[5]);
    }
}
