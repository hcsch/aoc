use std::collections::HashMap;
use std::lazy::SyncLazy;

use regex::Regex;

static CONTAINER_BAG_RE: SyncLazy<Regex> =
    SyncLazy::new(|| Regex::new(r"^(?P<name>[a-z ]+) bags contain").unwrap());
static CONTAINED_BAGS_RE: SyncLazy<Regex> =
    SyncLazy::new(|| Regex::new(r"(?P<count>[0-9]+) (?P<name>[a-z ]+) bags?(?:,|.)").unwrap());

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags_contains_map = HashMap::new();

    input_lines.for_each(|line| {
        let container_name = CONTAINER_BAG_RE
            .captures(&line)
            .expect("Line does not match expected format")
            .name("name")
            .unwrap()
            .as_str();

        if !bags_contains_map.contains_key(container_name) {
            bags_contains_map.insert(container_name.to_owned(), Vec::new());
        }

        CONTAINED_BAGS_RE.captures_iter(&line).for_each(|caps| {
            let count: usize = caps.name("count").unwrap().as_str().parse().unwrap();
            let bag_name = caps.name("name").unwrap().as_str();

            if !bags_contains_map.contains_key(bag_name) {
                bags_contains_map.insert(bag_name.to_owned(), Vec::new());
            }

            bags_contains_map
                .get_mut(container_name)
                .unwrap()
                .push((count, bag_name.to_owned()))
        });
    });

    bags_contains_map
}

fn can_contain_bag(
    bags_contains_map: &HashMap<String, Vec<(usize, String)>>,
    container_bag_name: &str,
    bag_name: &str,
) -> bool {
    let contained_bags = bags_contains_map.get(container_bag_name).unwrap();
    if contained_bags
        .iter()
        .any(|(_, contained_bag)| contained_bag == bag_name)
    {
        true
    } else {
        contained_bags
            .iter()
            .any(|(_, contained_bag)| can_contain_bag(bags_contains_map, contained_bag, bag_name))
    }
}

fn count_contained_bags(
    bags_contains_map: &HashMap<String, Vec<(usize, String)>>,
    container_bag_name: &str,
) -> usize {
    let mut computed_counts = HashMap::new();
    count_contained_bags_helper(&mut computed_counts, bags_contains_map, container_bag_name)
}

fn count_contained_bags_helper(
    computed_counts: &mut HashMap<String, usize>,
    bags_contains_map: &HashMap<String, Vec<(usize, String)>>,
    container_bag_name: &str,
) -> usize {
    let contained_bags_count = bags_contains_map
        .get(container_bag_name)
        .unwrap()
        .iter()
        .map(|(count, contained_bag)| {
            count
                + count
                    * computed_counts
                        .get(contained_bag)
                        .map(|contained_count| *contained_count)
                        .unwrap_or_else(|| {
                            count_contained_bags_helper(
                                computed_counts,
                                bags_contains_map,
                                contained_bag,
                            )
                        })
        })
        .sum();
    computed_counts.insert(container_bag_name.to_owned(), contained_bags_count);
    contained_bags_count
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bags_contains_map = parse_input(input_lines);
    bags_contains_map
        .keys()
        .filter(|container_bag_name| {
            can_contain_bag(&bags_contains_map, container_bag_name, "shiny gold")
        })
        .count()
        .to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bags_contains_map = parse_input(input_lines);
    count_contained_bags(&bags_contains_map, "shiny gold").to_string()
}
