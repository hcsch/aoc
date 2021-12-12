use std::collections::HashMap;

// Algorithm adapted from
// https://github.com/petgraph/petgraph/blob/9ff688872b467d3e1b5adef19f5c52f519d3279c/src/algo/simple_paths.rs#L36-L102

struct UnGraph<'a> {
    adjacency_map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> UnGraph<'a> {
    pub fn new() -> Self {
        UnGraph {
            adjacency_map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node_a: &'a str, node_b: &'a str) {
        self.adjacency_map
            .entry(node_a)
            .or_insert(Vec::new())
            .push(node_b);
        self.adjacency_map
            .entry(node_b)
            .or_insert(Vec::new())
            .push(node_a);
    }

    pub fn from_edges<I: Iterator<Item = (&'a str, &'a str)>>(iterator: I) -> Self {
        let mut graph = Self::new();
        for (node_a, node_b) in iterator {
            graph.add_edge(node_a, node_b);
        }
        graph
    }

    pub fn neighbors(&'a self, node: &str) -> impl 'a + Iterator<Item = &'a str> {
        self.adjacency_map
            .get(node)
            .map(|neighbors| neighbors as &[&'a str])
            .unwrap_or(&[])
            .into_iter()
            .copied()
    }
}

fn parse_input<'a>(input_lines: &'a Vec<String>) -> UnGraph<'a> {
    UnGraph::from_edges(input_lines.iter().map(|line| line.split_once('-').unwrap()))
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let input_lines = input_lines.collect();
    let cave_graph = parse_input(&input_lines);

    let mut num_paths: usize = 0;

    let mut children_stack = vec![cave_graph.neighbors("start")];
    let mut visited = vec!["start"];

    while let Some(children) = children_stack.last_mut() {
        if let Some(child) = children.next() {
            if child == "end" {
                num_paths += 1;
                continue;
            } else if child.bytes().all(|b| (b'A'..b'Z').contains(&b)) {
                visited.push(child);
                children_stack.push(cave_graph.neighbors(child));
            } else if !visited.contains(&child) {
                visited.push(child);
                children_stack.push(cave_graph.neighbors(child));
            } else {
                // Would visit a small cave a second time in this path
                // → track back one step and carry on.
                continue;
            }
        } else {
            // Done with all the children of this node, track back one step.
            children_stack.pop();
            visited.pop();
        }
    }

    num_paths.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let input_lines = input_lines.collect();
    let cave_graph = parse_input(&input_lines);

    let mut num_paths: usize = 0;

    let mut children_stack = vec![cave_graph.neighbors("start")];
    let mut path_stack = vec!["start"];
    let mut visited = vec!["start"];

    let mut small_cave_visited_twice = None;

    while let Some(children) = children_stack.last_mut() {
        if let Some(child) = children.next() {
            if child == "end" {
                num_paths += 1;
                continue;
            } else if child.bytes().all(|b| (b'A'..b'Z').contains(&b)) {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
            } else if !visited.contains(&child) {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
            } else if small_cave_visited_twice.is_none() && child != "start" {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
                small_cave_visited_twice = Some(child);
            } else {
                // Would visit more than one small cave a second time in this path
                // → track back one step and carry on.
                continue;
            }
        } else {
            // Done with all the children of this node, track back one step.
            children_stack.pop();
            visited.pop();
            if let Some(predecessor) = path_stack.pop() {
                if small_cave_visited_twice.map_or(false, |small_cave| small_cave == predecessor) {
                    small_cave_visited_twice = None;
                }
            }
        }
    }

    num_paths.to_string()
}
