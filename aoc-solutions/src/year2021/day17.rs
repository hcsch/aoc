#[derive(Clone, Copy)]
struct Target {
    pub min: [i16; 2],
    pub max: [i16; 2],
}

fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> Target {
    let first_line = input_lines.next().unwrap();
    let first_line = first_line.strip_prefix("target area: x=").unwrap();
    let (x_range_str, y_range_str) = first_line.split_once(", y=").unwrap();
    let (min_x, max_x) = x_range_str.split_once("..").unwrap();
    let (min_y, max_y) = y_range_str.split_once("..").unwrap();

    Target {
        min: [min_x.parse().unwrap(), min_y.parse().unwrap()],
        max: [max_x.parse().unwrap(), max_y.parse().unwrap()],
    }
}

struct Probe {
    pos: [i16; 2],
    velocity: [i16; 2],
}

impl Probe {
    pub fn from_initial_velocity(velocity: [i16; 2]) -> Self {
        Self {
            pos: [0; 2],
            velocity,
        }
    }

    pub const fn pos(&self) -> [i16; 2] {
        self.pos
    }

    pub fn step(&mut self) {
        self.pos[0] += self.velocity[0];

        self.pos[1] += self.velocity[1];

        self.velocity[0] = if self.velocity[0] > 0 {
            self.velocity[0] - 1
        } else if self.velocity[0] < 0 {
            self.velocity[0] + 1
        } else {
            self.velocity[0]
        };

        self.velocity[1] -= 1;
    }

    pub fn in_target(&self, target: &Target) -> bool {
        self.pos[0] >= target.min[0]
            && self.pos[1] >= target.min[1]
            && self.pos[0] <= target.max[0]
            && self.pos[1] <= target.max[1]
    }

    pub fn beyond_target(&self, target: &Target) -> bool {
        self.pos[1] < target.min[1]
    }
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let target = parse_input(input_lines);

    (0..=target.max[0])
        .flat_map(|x| {
            (target.min[1]..=200).filter_map(move |y| {
                let mut probe = Probe::from_initial_velocity([x, y]);
                let mut max_height = 0;
                loop {
                    if probe.in_target(&target) {
                        break Some(max_height);
                    } else if probe.beyond_target(&target) {
                        break None;
                    }
                    probe.step();
                    max_height = max_height.max(probe.pos()[1]);
                }
            })
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let target = parse_input(input_lines);

    (0..=target.max[0])
        .flat_map(|x| {
            (target.min[1]..=200).filter_map(move |y| {
                let mut probe = Probe::from_initial_velocity([x, y]);
                loop {
                    if probe.in_target(&target) {
                        break Some([x, y]);
                    } else if probe.beyond_target(&target) {
                        break None;
                    }
                    probe.step();
                }
            })
        })
        .count()
        .to_string()
}
