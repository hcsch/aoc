use std::{cmp::Ordering, collections::BinaryHeap};

use array_init;

const TILE_WIDTH: usize = 100;
const TILE_HEIGHT: usize = 100;

const X_TILES: usize = 5;
const Y_TILES: usize = 5;

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> [[u8; TILE_WIDTH]; TILE_HEIGHT] {
    let risk_levels = array_init::from_iter(input_lines.map(|line| {
        array_init::from_iter(line.into_bytes().into_iter().map(|b| b - b'0')).unwrap()
    }))
    .unwrap();

    risk_levels
}

trait Map {
    fn get(&self, index: [u16; 2]) -> u8;
}

struct TiledMap {
    base_tile: [[u8; TILE_WIDTH]; TILE_HEIGHT],
}

impl Map for TiledMap {
    fn get(&self, index: [u16; 2]) -> u8 {
        let (base_x, base_y) = (
            (index[1] as usize % TILE_WIDTH),
            (index[0] as usize % TILE_HEIGHT),
        );
        let (tile_x, tile_y) = (
            (index[1] / TILE_WIDTH as u16) as u8,
            (index[0] / TILE_HEIGHT as u16) as u8,
        );
        (self.base_tile[base_y][base_x] - 1 + (tile_x + tile_y) as u8) % 9 + 1
    }
}

impl<const TILE_WIDTH: usize, const TILE_HEIGHT: usize> Map for [[u8; TILE_WIDTH]; TILE_HEIGHT] {
    fn get(&self, index: [u16; 2]) -> u8 {
        self[index[0] as usize][index[1] as usize]
    }
}

// Algorithm adapted from
// https://github.com/rust-lang/rust/blob/d594910a2da12f158477b4c7281716f535cfa3de/library/alloc/src/collections/binary_heap.rs#L20-L95

#[derive(Clone, Copy, PartialEq, Eq)]
struct DijkstraState {
    risk: u16,
    position: [u16; 2],
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

fn neighbors(map_dim: [usize; 2], position: [u16; 2]) -> impl Iterator<Item = [u16; 2]> {
    [
        // above
        position[0].checked_sub(1).map(|y| [y, position[1]]),
        // left
        position[1].checked_sub(1).map(|x| [position[0], x]),
        // below
        position[0]
            .checked_add(1)
            .filter(|&y| y < map_dim[0] as u16)
            .map(|y| [y, position[1]]),
        // right
        position[1]
            .checked_add(1)
            .filter(|&x| x < map_dim[1] as u16)
            .map(|x| [position[0], x]),
    ]
    .into_iter()
    .filter_map(|neighboring_pos| neighboring_pos)
}

fn dijkstra_min_risk<M: Map, const MAP_WIDTH: usize, const MAP_HEIGHT: usize>(risk_map: M) -> u16 {
    const START: [u16; 2] = [0, 0];
    let end = [(MAP_HEIGHT - 1) as u16, (MAP_WIDTH - 1) as u16];

    let mut risk_of_path_to = [[u16::MAX; MAP_WIDTH]; MAP_HEIGHT];

    let mut heap = BinaryHeap::new();

    risk_of_path_to[START[0] as usize][START[1] as usize] = 0;
    heap.push(DijkstraState {
        risk: 0,
        position: START,
    });

    while let Some(DijkstraState { risk, position }) = heap.pop() {
        if position == end {
            return risk;
        }

        if risk > risk_of_path_to[position[0] as usize][position[1] as usize] {
            continue;
        }

        for neighboring_pos in neighbors([MAP_HEIGHT, MAP_WIDTH], position) {
            let next = DijkstraState {
                risk: risk + risk_map.get(neighboring_pos) as u16,
                position: neighboring_pos,
            };

            if next.risk < risk_of_path_to[next.position[0] as usize][next.position[1] as usize] {
                heap.push(next);
                risk_of_path_to[next.position[0] as usize][next.position[1] as usize] = next.risk;
            }
        }
    }

    unreachable!("path to goal must exist due to structure of the input for this puzzle")
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map = parse_input(input_lines);

    let solution = dijkstra_min_risk::<_, TILE_WIDTH, TILE_HEIGHT>(risk_map);

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let risk_map_tile = parse_input(input_lines);

    let solution =
        dijkstra_min_risk::<_, { X_TILES * TILE_WIDTH }, { Y_TILES * TILE_HEIGHT }>(TiledMap {
            base_tile: risk_map_tile,
        });

    solution.to_string()
}
