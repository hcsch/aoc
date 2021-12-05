use std::cmp;

struct Line {
    start: [u16; 2],
    end: [u16; 2],
}

impl Line {
    pub fn is_axis_aligned(&self) -> bool {
        self.start[0] == self.end[0] || self.start[1] == self.end[1]
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

    pub fn step(&self) -> [i8; 2] {
        [
            (self.end[0] as i32 - self.start[0] as i32).signum() as i8,
            (self.end[1] as i32 - self.start[1] as i32).signum() as i8,
        ]
    }
}

impl IntoIterator for &Line {
    type Item = <LineIterator as Iterator>::Item;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            current_pos: [i32::from(self.start[0]), i32::from(self.start[1])],
            step: self.step(),
            end: [i32::from(self.end[0]), i32::from(self.end[1])],
        }
    }
}

struct LineIterator {
    current_pos: [i32; 2],
    step: [i8; 2],
    end: [i32; 2],
}

impl Iterator for LineIterator {
    type Item = [u16; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.step[0] >= 0 && self.current_pos[0] > self.end[0]
            || self.step[0] < 0 && self.current_pos[0] < self.end[0]
            || self.step[1] >= 0 && self.current_pos[1] > self.end[1]
            || self.step[1] < 0 && self.current_pos[1] < self.end[1]
        {
            return None;
        }

        let current_pos = self.current_pos;

        self.current_pos[0] += self.step[0] as i32;
        self.current_pos[1] += self.step[1] as i32;

        Some([
            u16::try_from(current_pos[0]).unwrap(),
            u16::try_from(current_pos[1]).unwrap(),
        ])
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
        for [x, y] in line {
            ocean_floor[x as usize + y as usize * (max_x + 1) as usize] += 1;
        }
    }

    let solution = ocean_floor.iter().filter(|&&x| x >= 2).count();

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let vent_lines = parse_input(input_lines);

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
        for [x, y] in line {
            ocean_floor[x as usize + y as usize * (max_x + 1) as usize] += 1;
        }
    }

    let solution = ocean_floor.iter().filter(|&&x| x >= 2).count();

    solution.to_string()
}
