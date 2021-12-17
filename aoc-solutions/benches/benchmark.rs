use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

use aoc_solutions::*;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use paste::paste;

fn read_input_lines(year: u32, day: u32) -> Vec<String> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(&format!("../res/year{year:04}/input_day{day:02}.txt"));

    BufReader::new(
        fs::OpenOptions::new()
            .read(true)
            .open(&input_path)
            .expect(&format!(
                "Could not open input file {input_path:?} for reading"
            )),
    )
    .lines()
    .map(|l| l.expect("Error occurred while reading lines from input"))
    .collect()
}

macro_rules! bench_puzzles {
    ($(($year:literal, [$($day:literal),+])),+) => {
        $(
            paste! {
                $(
                    pub fn [<bench_year $year _day $day>](c: &mut Criterion) {
                        let mut group = c.benchmark_group(concat!("year ", stringify!($year), " day ", stringify!($day)));

                        let input_lines =
                            read_input_lines($year, $day);

                        group.bench_function("puzzle 1", |b| {
                            b.iter_batched(
                                || input_lines.clone().into_iter(),
                                |input| [<year $year>]::[<day $day>]::solve_puzzle1(black_box(input)),
                                BatchSize::SmallInput,
                            )
                        });

                        group.bench_function("puzzle 2", |b| {
                            b.iter_batched(
                                || input_lines.clone().into_iter(),
                                |input| [<year $year>]::[<day $day>]::solve_puzzle2(black_box(input)),
                                BatchSize::SmallInput,
                            )
                        });

                        group.finish();
                    }
                )+
                criterion_group!(
                    [<benches_year $year>],
                    $(
                        [<bench_year $year _day $day>]
                    ),+
                );
            }
        )+
    };
}

bench_puzzles!(
    (2020, [01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12]),
    (
        2021,
        [01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16]
    )
);

criterion_main!(benches_year2020, benches_year2021);
