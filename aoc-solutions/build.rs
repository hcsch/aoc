#![feature(format_args_capture)]

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    let src_dir = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src");

    let solved_days = fs::read_dir("src")
        .expect("Could not read src directory")
        .map(Result::unwrap)
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().starts_with("day")
        })
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .filter(|name| name.starts_with("day") && name.ends_with(".rs"))
        .map(|name| name[3..(name.len() - 3)].parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut source = String::new();

    for day in solved_days.iter() {
        source.push_str(&format!(
            "pub mod day{day:02}{{ include!(\"{day_file}\"); }}\n",
            day = day,
            day_file = src_dir
                .join(&format!("day{day:02}.rs", day = day))
                .to_str()
                .unwrap()
        ));
    }

    source.push_str(
        r#"pub fn solve_puzzle<I: Iterator<Item = String>>(day: usize, puzzle_number: usize, input_lines: I) -> String {
    #[allow(overlapping_patterns)]
    match (day, puzzle_number) {
"#,
    );

    for day in solved_days.iter() {
        for puzzle in 1..=2 {
            source.push_str(&format!(
                "        ({day}, {puzzle}) => day{day:02}::solve_puzzle{puzzle}(input_lines),\n",
                day = day,
                puzzle = puzzle
            ));
        }
    }

    source.push_str(
        r#"        (1..=25, 1..=2) => todo!("This puzzle has not yet been solved"),
        (day, 1..=2) => panic!("Invalid day {}! Must be between 1 and 25", day),
        (1..=25, puzzle) => panic!("Invalid puzzle number {}! Must be either 1 or 2", puzzle),
        (day, puzzle) => panic!("Invalid day {} and puzzle number {}! Must be between 1 and 25 and either 1 or 2 respectively", day, puzzle),
    }
}
"#,
    );

    fs::write(&dest_path, source).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
}
