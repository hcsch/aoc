use petgraph::graphmap::UnGraphMap;

fn parse_input(input_lines: &Vec<String>) -> UnGraphMap<&str, ()> {
    UnGraphMap::<&str, ()>::from_edges(input_lines.iter().map(|line| line.split_once('-').unwrap()))
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

    let mut small_cave_visited_twice = false;

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
            } else if !small_cave_visited_twice && child != "start" {
                visited.push(child);
                path_stack.push(child);
                children_stack.push(cave_graph.neighbors(child));
                small_cave_visited_twice = true;
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
                if predecessor.bytes().all(|b| (b'a'..b'z').contains(&b))
                    && visited.iter().find(|node| **node == predecessor).is_some()
                {
                    small_cave_visited_twice = false;
                }
            }
        }
    }

    num_paths.to_string()
}
