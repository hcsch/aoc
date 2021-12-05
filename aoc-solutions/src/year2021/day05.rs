use std::cmp;

struct Line {
    start: [u16; 2],
    end: [u16; 2],
}

impl Line {
    pub fn is_axis_aligned(&self) -> bool {
        self.start[0] == self.end[0] || self.start[1] == self.end[1]
    }

    pub fn min_x(&self) -> u16 {
        cmp::min(self.start[0], self.end[0])
    }

    pub fn min_y(&self) -> u16 {
        cmp::min(self.start[1], self.end[1])
    }

    pub fn max_x(&self) -> u16 {
        cmp::max(self.start[0], self.end[0])
    }

    pub fn max_y(&self) -> u16 {
        cmp::max(self.start[1], self.end[1])
    }

    pub fn max_coords(&self) -> [u16; 2] {
        [self.max_x(), self.max_y()]
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<Line> {
    input_lines
        .map(|line_str| {
            let (pre_arrow_str, post_arrow_str) = line_str
                .split_once(" -> ")
                .expect("expected vent line description, found no arrow in line");
            let (start_x_str, start_y_str) = pre_arrow_str
                .split_once(',')
                .expect("expected comma separated start values");
            let (end_x_str, end_y_str) = post_arrow_str
                .split_once(',')
                .expect("expected comma separated end values");

            Line {
                start: [
                    start_x_str.parse().expect("expected unsigned int x value"),
                    start_y_str.parse().expect("expected unsigned int y value"),
                ],
                end: [
                    end_x_str.parse().expect("expected unsigned int x value"),
                    end_y_str.parse().expect("expected unsigned int y value"),
                ],
            }
        })
        .collect()
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let mut vent_lines = parse_input(input_lines);

    vent_lines.retain(Line::is_axis_aligned);

    let [max_x, max_y] = vent_lines.iter().map(Line::max_coords).fold(
        [0; 2],
        |[global_max_x, global_max_y], [line_max_x, line_max_y]| {
            [
                cmp::max(global_max_x, line_max_x),
                cmp::max(global_max_y, line_max_y),
            ]
        },
    );

    let mut ocean_floor = vec![0u8; (max_x + 1) as usize * (max_y + 1) as usize];

    for line in &vent_lines {
        for y in line.min_y()..=line.max_y() {
            for x in line.min_x()..=line.max_x() {
                ocean_floor[x as usize + y as usize * (max_x + 1) as usize] += 1;
            }
        }
    }

    let solution = ocean_floor.iter().filter(|&&x| x >= 2).count();

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    todo!()
}
