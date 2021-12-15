use std::{cmp::Ordering, collections::BinaryHeap};

use ndarray::{self, Array2, Axis};

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Array2<u8> {
    let mut width = None;
    let raw_risk_levels: Vec<u8> = input_lines
        .flat_map(|line| {
            if let Some(width) = width {
                if width != line.len() {
                    panic!("not all lines of same length");
                }
            } else {
                width = Some(line.len());
            }
            line.into_bytes().into_iter().map(|b| b - b'0')
        })
        .collect();

    let width = width.expect("expected non-empty input file");

    Array2::from_shape_vec((raw_risk_levels.len() / width, width), raw_risk_levels).unwrap()
}

// Algorithm adapted from
// https://github.com/rust-lang/rust/blob/d594910a2da12f158477b4c7281716f535cfa3de/library/alloc/src/collections/binary_heap.rs#L20-L95

#[derive(Clone, Copy, PartialEq, Eq)]
struct DijkstraState {
    risk: usize,
    position: (usize, usize),
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk
            .cmp(&other.risk)
            .reverse() // minimal risk is "greater"
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(
    map_dim: ndarray::Dim<[usize; 2]>,
    position: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [
        // above
        position.0.checked_sub(1).map(|y| (y, position.1)),
        // left
        position.1.checked_sub(1).map(|x| (position.0, x)),
        // below
        position
            .0
            .checked_add(1)
            .filter(|&y| y < map_dim[0])
            .map(|y| (y, position.1)),
        // right
        position
            .1
            .checked_add(1)
            .filter(|&x| x < map_dim[1])
            .map(|x| (position.0, x)),
    ]
    .into_iter()
    .filter_map(|neighboring_pos| neighboring_pos)
}

fn dijkstra_min_risk(risk_map: &Array2<u8>) -> usize {
    const START: (usize, usize) = (0, 0);
    let end = (risk_map.nrows() - 1, risk_map.ncols() - 1);

    let mut risk_of_path_to = risk_map.map(|_| usize::MAX);

    let mut heap = BinaryHeap::new();

    risk_of_path_to[START] = 0;
    heap.push(DijkstraState {
        risk: 0,
        position: START,
    });

    while let Some(DijkstraState { risk, position }) = heap.pop() {
        if position == end {
            return risk;
        }

        if risk > risk_of_path_to[position] {
            continue;
        }

        for neighboring_pos in neighbors(risk_map.raw_dim(), position) {
            let next = DijkstraState {
                risk: risk + risk_map[neighboring_pos] as usize,
                position: neighboring_pos,
            };

            if next.risk < risk_of_path_to[next.position] {
                heap.push(next);
                risk_of_path_to[next.position] = next.risk;
            }
        }
    }

    unreachable!("path to goal must exist due to structure of the input for this puzzle")
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map = parse_input(input_lines);

    let solution = dijkstra_min_risk(&risk_map);

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map = parse_input(input_lines);

    // Concatenate mapped tile into 5 tile row.
    let risk_map = ndarray::concatenate(
        Axis(1),
        &[
            risk_map.view(),
            risk_map.mapv(|b| b % 9 + 1).view(),
            risk_map.mapv(|b| (b + 1) % 9 + 1).view(),
            risk_map.mapv(|b| (b + 2) % 9 + 1).view(),
            risk_map.mapv(|b| (b + 3) % 9 + 1).view(),
        ],
    )
    .unwrap();

    // Concatenate mapped rows into 5x5 tile grid.
    let risk_map = ndarray::concatenate(
        Axis(0),
        &[
            risk_map.view(),
            risk_map.mapv(|b| b % 9 + 1).view(),
            risk_map.mapv(|b| (b + 1) % 9 + 1).view(),
            risk_map.mapv(|b| (b + 2) % 9 + 1).view(),
            risk_map.mapv(|b| (b + 3) % 9 + 1).view(),
        ],
    )
    .unwrap();

    let solution = dijkstra_min_risk(&risk_map);

    solution.to_string()
}
