use std::collections::HashSet;

use ndarray::Array2;

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Array2<u8> {
    let mut width = None;
    let raw_heights: Vec<u8> = input_lines
        .flat_map(|line| {
            if let Some(width) = width {
                assert_eq!(line.len(), width);
            } else {
                width = Some(line.len());
            }
            line.bytes().map(|b| b - b'0').collect::<Vec<_>>()
        })
        .collect();

    let width = width.unwrap();

    Array2::from_shape_vec((width, raw_heights.len() / width), raw_heights).unwrap()
}

fn neighbors(
    map_dim: ndarray::Dim<[usize; 2]>,
    index: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [
        index.0.checked_sub(1).map(|x| (x, index.1)),
        index.1.checked_sub(1).map(|y| (index.0, y)),
        index
            .0
            .checked_add(1)
            .filter(|&x| x < map_dim[0])
            .map(|x| (x, index.1)),
        index
            .1
            .checked_add(1)
            .filter(|&y| y < map_dim[1])
            .map(|y| (index.0, y)),
    ]
    .into_iter()
    .filter_map(|neighboring_height| neighboring_height)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let height_map = parse_input(input_lines);

    let solution: usize = height_map
        .indexed_iter()
        .filter(|&(index, &height)| {
            neighbors(height_map.raw_dim(), index)
                .all(|neighboring_index| height < height_map[neighboring_index])
        })
        .map(|(_index, height)| (height + 1) as usize)
        .sum::<usize>();

    solution.to_string()
}

fn basin_size_of_low_point(height_map: &Array2<u8>, low_point: (usize, usize)) -> usize {
    let mut to_process = HashSet::new();
    let mut in_basin = HashSet::new();

    in_basin.insert(low_point);
    to_process.extend(neighbors(height_map.raw_dim(), low_point));

    while !to_process.is_empty() {
        let mut new_to_process = HashSet::new();
        for point in to_process.drain() {
            if height_map[point] == 9 {
                continue;
            }
            in_basin.insert(point);
            new_to_process.extend(
                neighbors(height_map.raw_dim(), point).filter(|point| !in_basin.contains(point)),
            );
        }
        to_process = new_to_process;
    }

    in_basin.len()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let height_map = parse_input(input_lines);

    let mut basins: Vec<usize> = height_map
        .indexed_iter()
        .filter(|&(index, &height)| {
            neighbors(height_map.raw_dim(), index)
                .all(|neighboring_index| height < height_map[neighboring_index])
        })
        .map(|(low_point, _height)| basin_size_of_low_point(&height_map, low_point))
        .collect();

    basins.sort_unstable();

    basins[basins.len() - 3..]
        .iter()
        .product::<usize>()
        .to_string()
}
