#![feature(bool_to_option, once_cell)]

use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::{crate_authors, crate_version, Parser};

use aoc_solutions::*;

fn open_input_reader<P: AsRef<Path>>(path_or_dash: P) -> io::Result<Box<dyn BufRead>> {
    let path_or_dash = path_or_dash.as_ref();
    if path_or_dash.as_os_str() == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        fs::OpenOptions::new()
            .read(true)
            .open(path_or_dash)
            .map(BufReader::new)
            .map(Box::new)
            .map(|reader| reader as Box<dyn BufRead>)
    }
}

fn is_valid_day(v: &str) -> Result<(), String> {
    v.parse::<u8>()
        .ok()
        .and_then(|d| (1..25).contains(&d).then_some(()))
        .ok_or("Not a valid day number (between 1 and 25)".to_owned())
}

#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Options {
    /// Path to the file containing the input or '-' for stdin.
    #[clap(short, long, parse(from_os_str), default_value = "-")]
    input: PathBuf,
    /// Year from which the puzzles should be selected
    #[clap(possible_values = &["2020", "2021"])]
    year: usize,
    /// Number of the day the puzzles of which should be used
    #[clap(validator(is_valid_day))]
    day: usize,
    /// Number of the puzzle which should be used from the given day
    #[clap(possible_values = &["1", "2"])]
    puzzle_number: usize,
}

fn main() {
    let Options {
        input,
        year,
        day,
        puzzle_number,
    } = Options::parse();

    let input_reader = open_input_reader(input).expect("Failed to open input file for reading");
    let input_lines = input_reader
        .lines()
        .map(|l| l.expect("Error occurred while reading lines from input"));

    let solution = solve_puzzle(year, day, puzzle_number, input_lines);

    println!("The solution to puzzle {puzzle_number} of day {day} is \"{solution}\"")
}
