use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

// Algorithm adapted from
// https://github.com/petgraph/petgraph/blob/9ff688872b467d3e1b5adef19f5c52f519d3279c/src/algo/simple_paths.rs#L36-L102

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(u16),
    Large(u16),
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(cave_str: &str) -> Result<Self, Self::Err> {
        match cave_str {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            small_cave if small_cave.bytes().all(|b| b.is_ascii_lowercase()) => Ok(Self::Small(
                small_cave
                    .bytes()
                    .fold(0, |acc, b| acc * 26 + (b - b'a') as u16),
            )),
            large_cave if large_cave.bytes().all(|b| b.is_ascii_uppercase()) => Ok(Self::Large(
                large_cave
                    .bytes()
                    .fold(0, |acc, b| acc * 26 + (b - b'A') as u16),
            )),
            _ => Err("expected `start`, `end`, a lowercase cave name, an uppercase cave name"),
        }
    }
}

struct Graph {
    adjacency_map: HashMap<Cave, HashSet<Cave>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node_a: Cave, node_b: Cave) {
        if node_a != Cave::End && node_b != Cave::Start {
            self.adjacency_map
                .entry(node_a)
                .or_insert(HashSet::new())
                .insert(node_b);
        }
        if node_b != Cave::End && node_a != Cave::Start {
            self.adjacency_map
                .entry(node_b)
                .or_insert(HashSet::new())
                .insert(node_a);
        }
    }

    pub fn from_edges<I: Iterator<Item = (Cave, Cave)>>(iterator: I) -> Self {
        let mut graph = Self::new();
        for (node_a, node_b) in iterator {
            graph.add_edge(node_a, node_b);
        }
        graph
    }

    pub fn neighbors<'a>(&'a self, node: Cave) -> impl 'a + Iterator<Item = Cave> {
        self.adjacency_map
            .get(&node)
            .map(|neighbors| neighbors.iter().copied())
            .into_iter()
            .flatten()
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Graph {
    Graph::from_edges(input_lines.map(|line| {
        let (cave_a_str, cave_b_str) = line.split_once('-').unwrap();
        (cave_a_str.parse().unwrap(), cave_b_str.parse().unwrap())
    }))
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let cave_graph = parse_input(input_lines);

    let mut num_paths: usize = 0;

    let mut children_stack = vec![cave_graph.neighbors(Cave::Start)];
    let mut visited = vec![Cave::Start];

    while let Some(children) = children_stack.last_mut() {
        match children.next() {
            Some(Cave::End) => num_paths += 1,
            Some(child @ Cave::Large(_)) => {
                visited.push(child);
                children_stack.push(cave_graph.neighbors(child));
            }
            Some(child @ Cave::Small(_)) if !visited.contains(&child) => {
                visited.push(child);
                children_stack.push(cave_graph.neighbors(child));
            }
            Some(Cave::Small(_) | Cave::Start) => {
                // Would visit a small cave or start a second time in this path
                // → skip.
            }
            None => {
                // Done with all the children of this node, track back one step.
                children_stack.pop();
                visited.pop();
            }
        }
    }

    num_paths.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let cave_graph = parse_input(input_lines);

    let mut num_paths: usize = 0;

    let mut children_stack = vec![cave_graph.neighbors(Cave::Start)];
    let mut path_stack = vec![Cave::Start];
    let mut visited = vec![Cave::Start];

    let mut small_cave_visited_twice = None;

    while let Some(children) = children_stack.last_mut() {
        match children.next() {
            Some(Cave::End) => num_paths += 1,
            Some(child @ Cave::Large(_)) => {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
            }
            Some(child @ Cave::Small(_)) if !visited.contains(&child) => {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
            }
            Some(child @ Cave::Small(_)) if small_cave_visited_twice.is_none() => {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
                small_cave_visited_twice = Some(child);
            }
            Some(Cave::Small(_) | Cave::Start) => {
                // Would visit more than one small cave or start a second time in this path
                // → skip.
            }
            None => {
                // Done with all the children of this node, track back one step.
                children_stack.pop();
                visited.pop();
                if let Some(predecessor) = path_stack.pop() {
                    if small_cave_visited_twice
                        .map_or(false, |small_cave| small_cave == predecessor)
                    {
                        small_cave_visited_twice = None;
                    }
                }
            }
        }
    }

    num_paths.to_string()
}
