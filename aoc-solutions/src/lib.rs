#![feature(bool_to_option, once_cell)]

use paste::paste;

pub mod year2020;
pub mod year2021;

macro_rules! gen_solve_puzzle_fn {
    ($(($year:literal, [$($day:literal),+])),+) => {
        pub fn solve_puzzle<I: Iterator<Item = String>>(
            year: usize,
            day: usize,
            puzzle_number: usize,
            input_lines: I,
        ) -> String {
            paste! {
                match (year, day, puzzle_number) {
                    $(
                        $(
                            ($year, $day, 1) => [<year $year>]::[<day $day>]::solve_puzzle1(input_lines),
                            ($year, $day, 2) => [<year $year>]::[<day $day>]::solve_puzzle2(input_lines),
                        )+
                    )+
                    (_, 1..=25, 1..=2) => todo!("This puzzle has not yet been solved"),
                    (_, day, 1..=2) => panic!("Invalid day {}! Must be between 1 and 25", day),
                    (_, 1..=25, puzzle) => panic!("Invalid puzzle number {}! Must be either 1 or 2", puzzle),
                    (_, day, puzzle) => panic!("Invalid day {} and puzzle number {}! Must be between 1 and 25 and either 1 or 2 respectively", day, puzzle),
                }
            }
        }
    };
}

gen_solve_puzzle_fn!(
    (2020, [01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12]),
    (2021, [01, 02, 03, 04, 05])
);
