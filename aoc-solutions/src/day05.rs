use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum RowPart {
    Front,
    Back,
}

#[derive(Debug, Copy, Clone)]
enum ColPart {
    Left,
    Right,
}

fn parse_input<I: Iterator<Item = String>>(
    input_lines: I,
) -> impl Iterator<Item = ([RowPart; 7], [ColPart; 3])> {
    input_lines.map(|line| {
        let (row_part_str, col_part_str) = line.split_at(7);
        let mut row_part = [RowPart::Front; 7];
        for (i, c) in row_part_str.as_bytes().iter().enumerate() {
            row_part[i] = match c {
                b'F' => RowPart::Front,
                b'B' => RowPart::Back,
                _ => panic!("Invalid input format"),
            }
        }
        let mut col_part = [ColPart::Left; 3];
        for (i, c) in col_part_str.as_bytes().iter().enumerate() {
            col_part[i] = match c {
                b'L' => ColPart::Left,
                b'R' => ColPart::Right,
                _ => panic!("Invalid input format"),
            }
        }
        (row_part, col_part)
    })
}

fn compute_seat_details(row_part: [RowPart; 7], col_part: [ColPart; 3]) -> (u8, u8, u32) {
    let row_num = {
        let mut row_range = 0..128u8;
        let mut level = 128 / 2;
        for rp in row_part.iter() {
            match rp {
                RowPart::Front => row_range.end -= level,
                RowPart::Back => row_range.start += level,
            }
            level /= 2;
        }
        row_range.start
    };
    let col_num = {
        let mut col_range = 0..8u8;
        let mut level = 8 / 2;
        for cp in col_part.iter() {
            match cp {
                ColPart::Left => col_range.end -= level,
                ColPart::Right => col_range.start += level,
            }
            level /= 2;
        }
        col_range.start
    };
    (row_num, col_num, row_num as u32 * 8 + col_num as u32)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let seats = parse_input(input_lines);
    seats
        .map(|(row_part, col_part)| compute_seat_details(row_part, col_part))
        .map(|(_, _, seat_id)| seat_id)
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let seats = parse_input(input_lines);
    seats
        .map(|(row_part, col_part)| compute_seat_details(row_part, col_part))
        .filter(|(row, _, _)| *row > 0 || *row < 127)
        .map(|(_, _, id)| id)
        .sorted()
        .collect::<Vec<u32>>()
        .windows(2)
        .filter_map(|window| match window {
            &[current_id, next_id] if current_id + 1 < next_id => Some((current_id + 1)..next_id),
            _ => None,
        })
        .flatten()
        .next()
        .unwrap()
        .to_string()
}

// pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
//     let seats = parse_input(input_lines);
//     let mut last_id = 0;
//     for (_, _, id) in seats
//         .map(|(row_part, col_part)| compute_seat_details(row_part, col_part))
//         .sorted_by_key(|(_, _, id)| *id)
//     {
//         if last_id + 1 < id {
//             for missing_id in (last_id + 1)..id {
//                 println!(
//                     "row: {}, col: {}, id: {}",
//                     missing_id / 8,
//                     missing_id % 8,
//                     missing_id
//                 );
//             }
//         }
//         last_id = id;
//     }
//     "one of the above".to_owned()
// }
