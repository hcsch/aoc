use std::{cmp::Ordering, collections::BinaryHeap};

use array_init;

const TILE_WIDTH: usize = 100;
const TILE_HEIGHT: usize = 100;

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> [[u8; TILE_WIDTH]; TILE_HEIGHT] {
    let risk_levels = array_init::from_iter(input_lines.map(|line| {
        array_init::from_iter(line.into_bytes().into_iter().map(|b| b - b'0')).unwrap()
    }))
    .unwrap();

    risk_levels
}

// Algorithm adapted from
// https://github.com/rust-lang/rust/blob/d594910a2da12f158477b4c7281716f535cfa3de/library/alloc/src/collections/binary_heap.rs#L20-L95

#[derive(Clone, Copy, PartialEq, Eq)]
struct DijkstraState {
    risk: u16,
    position: [usize; 2],
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk
            .cmp(&other.risk)
            .reverse() // minimal risk is "greater"
            .then_with(|| self.position[0].cmp(&other.position[0]))
            .then_with(|| self.position[1].cmp(&other.position[1]))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(map_dim: [usize; 2], position: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    [
        // above
        position[0].checked_sub(1).map(|y| [y, position[1]]),
        // left
        position[1].checked_sub(1).map(|x| [position[0], x]),
        // below
        position[0]
            .checked_add(1)
            .filter(|&y| y < map_dim[0])
            .map(|y| [y, position[1]]),
        // right
        position[1]
            .checked_add(1)
            .filter(|&x| x < map_dim[1])
            .map(|x| [position[0], x]),
    ]
    .into_iter()
    .filter_map(|neighboring_pos| neighboring_pos)
}

fn dijkstra_min_risk<const MAP_WIDTH: usize, const MAP_HEIGHT: usize>(
    risk_map: [[u8; MAP_WIDTH]; MAP_HEIGHT],
) -> u16 {
    const START: [usize; 2] = [0, 0];
    let end = [MAP_HEIGHT - 1, MAP_WIDTH - 1];

    let mut risk_of_path_to = [[u16::MAX; MAP_WIDTH]; MAP_HEIGHT];

    let mut heap = BinaryHeap::new();

    risk_of_path_to[START[0]][START[1]] = 0;
    heap.push(DijkstraState {
        risk: 0,
        position: START,
    });

    while let Some(DijkstraState { risk, position }) = heap.pop() {
        if position == end {
            return risk;
        }

        if risk > risk_of_path_to[position[0]][position[1]] {
            continue;
        }

        for neighboring_pos in neighbors([MAP_HEIGHT, MAP_WIDTH], position) {
            let next = DijkstraState {
                risk: risk + risk_map[neighboring_pos[0]][neighboring_pos[1]] as u16,
                position: neighboring_pos,
            };

            if next.risk < risk_of_path_to[next.position[0]][next.position[1]] {
                heap.push(next);
                risk_of_path_to[next.position[0]][next.position[1]] = next.risk;
            }
        }
    }

    unreachable!("path to goal must exist due to structure of the input for this puzzle")
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map = parse_input(input_lines);

    let solution = dijkstra_min_risk(risk_map);

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map_tile = parse_input(input_lines);

    // Concatenate mapped tile into 5 tile row.
    let mut risk_map = [[0; TILE_WIDTH * 5]; TILE_HEIGHT * 5];
    for tile_y in 0..5 {
        for (row_i, row) in risk_map_tile.iter().enumerate() {
            for tile_x in 0..5 {
                for (dst, src) in risk_map[tile_y * TILE_HEIGHT + row_i]
                    [tile_x * TILE_WIDTH..(tile_x + 1) * TILE_WIDTH]
                    .iter_mut()
                    .zip(row)
                {
                    *dst = (src - 1 + (tile_x as u8 + tile_y as u8)) % 9 + 1;
                }
            }
        }
    }

    let solution = dijkstra_min_risk(risk_map);

    solution.to_string()
}
