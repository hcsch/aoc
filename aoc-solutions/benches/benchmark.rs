use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use aoc_solutions::*;

fn read_input_lines(qualified_puzzle_id: &str) -> Vec<String> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(&format!("../res/input_{}.txt", qualified_puzzle_id));

    BufReader::new(
        fs::OpenOptions::new()
            .read(true)
            .open(&input_path)
            .expect(&format!(
                "Could not open input file {:?} for reading",
                &input_path
            )),
    )
    .lines()
    .map(|l| l.expect("Error occurred while reading lines from input"))
    .collect()
}

macro_rules! bench_puzzles {
    ($(($bench_fn:ident, $sol1_fn:path, $sol2_fn:path, $day:expr)),+) => {
        $(
            pub fn $bench_fn(c: &mut Criterion) {
                let mut group =
                    c.benchmark_group(&format!("day {:02}", $day));

                let input_lines = read_input_lines(&format!(
                    "day{:02}",
                    $day
                ));

                group.bench_function(
                    "puzzle 1",
                    |b| b.iter_batched(|| {
                        input_lines.clone().into_iter()
                    }, |input| {
                        $sol1_fn(black_box(input))
                    }, BatchSize::SmallInput),
                );

                group.bench_function(
                    "puzzle 2",
                    |b| b.iter_batched(|| {
                        input_lines.clone().into_iter()
                    }, |input| {
                        $sol2_fn(black_box(input))
                    }, BatchSize::SmallInput),
                );

                group.finish();
            }
        )+
    };
}

bench_puzzles!(
    (bench_day01, day01::solve_puzzle1, day01::solve_puzzle2, 1),
    (bench_day02, day02::solve_puzzle1, day02::solve_puzzle2, 2),
    (bench_day03, day03::solve_puzzle1, day03::solve_puzzle2, 3),
    (bench_day04, day04::solve_puzzle1, day04::solve_puzzle2, 4),
    (bench_day05, day05::solve_puzzle1, day05::solve_puzzle2, 5),
    (bench_day06, day06::solve_puzzle1, day06::solve_puzzle2, 6),
    (bench_day07, day07::solve_puzzle1, day07::solve_puzzle2, 7),
    (bench_day08, day08::solve_puzzle1, day08::solve_puzzle2, 8),
    (bench_day09, day09::solve_puzzle1, day09::solve_puzzle2, 9),
    (bench_day10, day10::solve_puzzle1, day10::solve_puzzle2, 10),
    (bench_day11, day11::solve_puzzle1, day11::solve_puzzle2, 11)
);

criterion_group!(
    benches,
    bench_day01,
    bench_day02,
    bench_day03,
    bench_day04,
    bench_day05,
    bench_day06,
    bench_day07,
    bench_day08,
    bench_day09,
    bench_day10,
    bench_day11
);
criterion_main!(benches);
