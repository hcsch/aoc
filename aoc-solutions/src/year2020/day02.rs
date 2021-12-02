use std::lazy::SyncLazy;

use regex::Regex;

static PASSWORD_LIST_RE: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new(
        r"^(?P<min_usages>[0-9]+)-(?P<max_usages>[0-9]+) (?P<required_letter>[a-zA-Z]): (?P<password>[a-zA-Z]+)$",
    ).unwrap()
});

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> impl Iterator<Item = (usize, usize, String, String)> {
    input_lines.map(|line| {
        let caps = PASSWORD_LIST_RE
            .captures(&line)
            .expect("A line did not match the expected input format");
        (
            caps.name("min_usages")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            caps.name("max_usages")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            caps.name("required_letter").unwrap().as_str().to_owned(),
            caps.name("password").unwrap().as_str().to_owned(),
        )
    })
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    parse_input(input_lines)
        .filter(|(min_u, max_u, req_l, pw)| (*min_u..=*max_u).contains(&pw.matches(req_l).count()))
        .count()
        .to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    parse_input(input_lines)
        .filter(|(first_pos, second_pos, req_l, pw)| {
            let req_l = req_l.as_bytes()[0];
            let pw = pw.as_bytes();
            (pw[first_pos - 1] == req_l) ^ (pw[second_pos - 1] == req_l)
        })
        .count()
        .to_string()
}
