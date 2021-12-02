use std::{env, fmt::Write, fs, path::Path};

use indoc::indoc;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let src_dir = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src");

    let solved_years = fs::read_dir(src_dir)
        .expect("Could not read src directory")
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .filter_map(|entry| {
            let name = entry.file_name().into_string().unwrap();
            let stripped_name = name.strip_prefix("year")?;
            Some(
                (
                    entry.path(),
                    stripped_name.parse::<usize>()
                        .expect("found unexpected rust source directory with a name starting in `year` not followed by and only by a four digit number")
                )
            )
        })
        .collect::<Vec<_>>();

    let mut source = "use std::fmt::Display;\n".to_owned();
    let mut solved_days_with_year = vec![];

    for (path, year) in solved_years.iter() {
        let solved_days = fs::read_dir(path)
            .expect("Could not read src directory")
            .map(Result::unwrap)
            .filter(|entry| entry.file_type().unwrap().is_file())
            .filter_map(|entry| {
                let name = entry.file_name().into_string().unwrap();
                let stripped_name = name.strip_prefix("day").and_then(|name| name.strip_suffix(".rs"))?;
                Some(
                    stripped_name.parse::<usize>()
                        .expect("found unexpected rust source file with a name starting in `day` not followed by and only by a two digit number")
                )
            })
            .collect::<Vec<_>>();

        solved_days_with_year.extend(solved_days.iter().copied().map(|day| (year, day)));
    }

    source.push_str(indoc! {r#"
            pub fn solve_puzzle<I: Iterator<Item = String>>(
                year: usize,
                day: usize,
                puzzle_number: usize,
                input_lines: I,
            ) -> Box<dyn Display> {
                #[allow(overlapping_patterns)]
                match (year, day, puzzle_number) {
        "#});

    for (year, day) in solved_days_with_year.iter() {
        for puzzle in 1..=2 {
            writeln!(source, 
                "        ({year}, {day}, {puzzle}) => Box::new(year{year:04}::day{day:02}::solve_puzzle{puzzle}(input_lines)),\n",
                year = year,
                day = day,
                puzzle = puzzle
            ).unwrap();
        }
    }

    source.push_str(
        indoc!{r#"
                    (_, 1..=25, 1..=2) => todo!("This puzzle has not yet been solved"),
                    (_, day, 1..=2) => panic!("Invalid day {}! Must be between 1 and 25", day),
                    (_, 1..=25, puzzle) => panic!("Invalid puzzle number {}! Must be either 1 or 2", puzzle),
                    (_, day, puzzle) => panic!("Invalid day {} and puzzle number {}! Must be between 1 and 25 and either 1 or 2 respectively", day, puzzle),
                }
            }
        "#},
    );

    fs::write(&dest_path, source).unwrap();
    println!("cargo:rerun-if-changed=src");
}
