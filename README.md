# aoc
This is the repository for my solutions to the Advent of Code puzzles.

## aoc-solutions
The crate that contains the actual solutions to the puzzles.

## aoc-solver
The CLI tool for computing the solutions for a given puzzle with the code from `aoc-solutions`.

## Computing a solution
To compute a solution to the second puzzle of the 3rd day of 2021, for example,
you can use the following command:
```sh
cargo run -- --input res/year2021/input_day03.txt 2021 03 2
```

## Benchmarking a solution
To benchmark a particular solution, e.g. for 2021 day 3, puzzle 2, run
```sh
cargo criterion "year 2021 day 03/puzzle 2"
```
To benchmark all puzzle solutions, run
```sh
cargo criterion
```

You can also run
```sh
cargo bench "year 2021 day 03/puzzle 2"
```
but this will not generate a report HTML file in `target/criterion/reports/` anymore in the future (see bheisler/criterion.rs#426).

*Note: I haven't really tried to seriously optimize any of these solutions,
the use of criterion is more for fun in this case.*
