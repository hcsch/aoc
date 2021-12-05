use std::cmp;

struct Line {
    start: [u16; 2],
    end: [u16; 2],
}

impl Line {
    pub fn is_axis_aligned(&self) -> bool {
        self.start[0] == self.end[0] || self.start[1] == self.end[1]
    }

    fn max_coord_value(&self, i: usize) -> u16 {
        cmp::max(self.start[i], self.end[i])
    }

    pub fn max_coords(&self) -> [u16; 2] {
        [self.max_coord_value(0), self.max_coord_value(1)]
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
        // Check if current position is after `end` in the `step` direction.
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
            u16::try_from(current_pos[0]).expect("expected non-negative coordinates only"),
            u16::try_from(current_pos[1]).expect("expected non-negative coordinates only"),
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

fn max_coords_of_lines<'a, I: Iterator<Item = &'a Line>>(lines: I) -> [u16; 2] {
    lines.map(Line::max_coords).fold(
        [0; 2],
        |[global_max_x, global_max_y], [line_max_x, line_max_y]| {
            [
                cmp::max(global_max_x, line_max_x),
                cmp::max(global_max_y, line_max_y),
            ]
        },
    )
}

fn count_overlapping_points(lines: Vec<Line>) -> usize {
    let [max_x, max_y] = max_coords_of_lines(lines.iter());

    let ocean_floor_width = (max_x + 1) as usize;
    let ocean_floor_height = (max_y + 1) as usize;

    let mut ocean_floor = vec![0u8; ocean_floor_width * ocean_floor_height];

    for line in &lines {
        for [x, y] in line {
            ocean_floor[x as usize + y as usize * ocean_floor_width] += 1;
        }
    }

    ocean_floor.iter().filter(|&&x| x >= 2).count()
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let mut vent_lines = parse_input(input_lines);

    vent_lines.retain(Line::is_axis_aligned);

    let solution = count_overlapping_points(vent_lines);

    solution.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let vent_lines = parse_input(input_lines);

    let solution = count_overlapping_points(vent_lines);

    solution.to_string()
}
