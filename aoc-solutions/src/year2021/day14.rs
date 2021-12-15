use std::{collections::HashMap, hash::Hash};

use array_init;
use itertools::{Itertools, MinMaxResult};

struct Counts<Item> {
    map: HashMap<Item, u64>,
}

impl<Item> Counts<Item> {
    pub fn into_inner(self) -> HashMap<Item, u64> {
        self.map
    }
}

impl<Item> FromIterator<Item> for Counts<Item>
where
    Item: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut map = HashMap::new();
        iter.into_iter()
            .for_each(|item| *map.entry(item).or_insert(0) += 1);
        Self { map }
    }
}

fn parse_input<I: Iterator<Item = String>>(
    mut input_lines: I,
) -> (HashMap<[u8; 2], u64>, HashMap<[u8; 2], u8>) {
    let polymer_template_string = input_lines.next().unwrap();
    let mut pair_counts = polymer_template_string
        .as_bytes()
        .array_windows()
        .copied()
        .collect::<Counts<[u8; 2]>>()
        .into_inner();
    if let Some(&last) = polymer_template_string.as_bytes().last() {
        pair_counts.insert([last, 0], 1);
    }

    // Skip empty line.
    input_lines.next().unwrap();

    let insertion_rules = input_lines
        .map(|line| {
            let (from_str, to_str) = line.split_once(" -> ").unwrap();
            (
                array_init::from_iter(from_str.bytes()).unwrap(),
                <[u8; 1]>::try_from(to_str.as_bytes()).unwrap()[0],
            )
        })
        .collect();

    (pair_counts, insertion_rules)
}

fn run_polymerization(
    mut pair_counts: HashMap<[u8; 2], u64>,
    insertion_rules: HashMap<[u8; 2], u8>,
    steps: usize,
) -> u64 {
    let mut new_pair_counts: HashMap<[u8; 2], u64> = HashMap::new();

    for _ in 0..steps {
        for (&pair, &count) in pair_counts.iter() {
            if let Some(&to_insert) = insertion_rules.get(&pair) {
                *new_pair_counts.entry([pair[0], to_insert]).or_insert(0) += count;
                *new_pair_counts.entry([to_insert, pair[1]]).or_insert(0) += count;
            } else {
                *new_pair_counts.entry(pair).or_insert(0) += count;
            }
        }
        pair_counts.clear();
        pair_counts.extend(new_pair_counts.drain());
        new_pair_counts.clear();
    }

    let counts = pair_counts
        .iter()
        .fold(HashMap::new(), |mut counts, (&[element, _], &count)| {
            *counts.entry(element).or_insert(0) += count;
            counts
        });

    let solution = match counts.values().minmax() {
        MinMaxResult::NoElements => 0,
        MinMaxResult::OneElement(_) => 0,
        MinMaxResult::MinMax(min, max) => max - min,
    };

    solution
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (pair_counts, insertion_rules) = parse_input(input_lines);

    let solution = run_polymerization(pair_counts, insertion_rules, 10);

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (pair_counts, insertion_rules) = parse_input(input_lines);

    let solution = run_polymerization(pair_counts, insertion_rules, 40);

    solution.to_string()
}
