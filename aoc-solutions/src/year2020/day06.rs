use std::collections::HashSet;

use itertools::Itertools;

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> impl Iterator<Item = Vec<HashSet<char>>> {
    input_lines.batching(|it| {
        let group_customs_decls = it
            .take_while(|line| !line.is_empty())
            .map(|person_customs_decl| person_customs_decl.chars().collect())
            .collect::<Vec<HashSet<char>>>();

        if group_customs_decls.is_empty() {
            None
        } else {
            Some(group_customs_decls)
        }
    })
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let groups_customs_decls = parse_input(input_lines);
    groups_customs_decls
        .map(|mut group_customs_decls| {
            group_customs_decls
                .iter_mut()
                .reduce(|acc, person_customs_decl| {
                    acc.extend(person_customs_decl.iter());
                    acc
                })
                .map_or(0, |group_customs_decl| group_customs_decl.len())
        })
        .sum()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> usize {
    let groups_customs_decls = parse_input(input_lines);
    groups_customs_decls
        .map(|mut group_customs_decls| {
            group_customs_decls
                .iter_mut()
                .reduce(|acc, person_customs_decl| {
                    acc.retain(|c| person_customs_decl.contains(c));
                    acc
                })
                .map_or(0, |group_customs_decl| group_customs_decl.len())
        })
        .sum()
}
