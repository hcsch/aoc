use array_init;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OctopusState {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    Charged,
    Flashed,
}

impl OctopusState {
    pub fn increase_energy(self) -> Self {
        use OctopusState::*;
        match self {
            L0 => L1,
            L1 => L2,
            L2 => L3,
            L3 => L4,
            L4 => L5,
            L5 => L6,
            L6 => L7,
            L7 => L8,
            L8 => L9,
            L9 => Charged,
            Charged => Charged,
            Flashed => Flashed,
        }
    }
}

impl TryFrom<u8> for OctopusState {
    type Error = &'static str;

    fn try_from(energy_level: u8) -> Result<Self, Self::Error> {
        match energy_level {
            0 => Ok(OctopusState::L0),
            1 => Ok(OctopusState::L1),
            2 => Ok(OctopusState::L2),
            3 => Ok(OctopusState::L3),
            4 => Ok(OctopusState::L4),
            5 => Ok(OctopusState::L5),
            6 => Ok(OctopusState::L6),
            7 => Ok(OctopusState::L7),
            8 => Ok(OctopusState::L8),
            9 => Ok(OctopusState::L9),
            _ => Err("not a valid starting octopus energy level"),
        }
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> [OctopusState; WIDTH * HEIGHT] {
    array_init::from_iter(input_lines.flat_map(|line| {
        line.into_bytes()
            .into_iter()
            .map(|b| OctopusState::try_from(b - b'0').unwrap())
    }))
    .unwrap()
}

fn index1d_to_2d(index: usize) -> (usize, usize) {
    (index % WIDTH, index / WIDTH)
}

fn index2d_to_1d((x, y): (usize, usize)) -> usize {
    x + y * WIDTH
}

fn neighbors(index: usize) -> impl Iterator<Item = usize> {
    let (x, y) = index1d_to_2d(index);
    let neighboring_xs = [x.checked_sub(1), Some(x + 1).filter(|&x| x < WIDTH)];
    let neighboring_ys = [y.checked_sub(1), Some(y + 1).filter(|&y| y < HEIGHT)];
    [
        // row above
        neighboring_xs[0].and_then(|x| neighboring_ys[0].map(|y| (x, y))),
        neighboring_ys[0].map(|y| (x, y)),
        neighboring_xs[1].and_then(|x| neighboring_ys[0].map(|y| (x, y))),
        // same row
        neighboring_xs[0].map(|x| (x, y)),
        neighboring_xs[1].map(|x| (x, y)),
        // row below
        neighboring_xs[0].and_then(|x| neighboring_ys[1].map(|y| (x, y))),
        neighboring_ys[1].map(|y| (x, y)),
        neighboring_xs[1].and_then(|x| neighboring_ys[1].map(|y| (x, y))),
    ]
    .into_iter()
    .filter_map(|opt_i| opt_i)
    .map(index2d_to_1d)
}

fn step_octopus_population(
    mut octopus_states: [OctopusState; WIDTH * HEIGHT],
) -> (usize, [OctopusState; WIDTH * HEIGHT]) {
    let mut new_octopus_states = [OctopusState::L0; WIDTH * HEIGHT];

    for (new_octopus_state, octopus_state) in new_octopus_states.iter_mut().zip(octopus_states) {
        *new_octopus_state = octopus_state.increase_energy();
    }

    octopus_states = new_octopus_states;

    loop {
        for (i, _octopus_state) in octopus_states
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, octopus_state)| octopus_state == OctopusState::Charged)
        {
            new_octopus_states[i] = OctopusState::Flashed;
            for neighbor_i in neighbors(i) {
                new_octopus_states[neighbor_i] = new_octopus_states[neighbor_i].increase_energy();
            }
        }

        if new_octopus_states == octopus_states {
            break;
        }

        octopus_states = new_octopus_states;
    }

    let flashed_count = new_octopus_states
        .iter_mut()
        .filter(|state| **state == OctopusState::Flashed)
        .fold(0, |flashed_count, state| {
            *state = OctopusState::L0;
            flashed_count + 1
        });

    (flashed_count, new_octopus_states)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let octopus_states = parse_input(input_lines);

    let (flashed_count_total, _) = (0..100).fold(
        (0, octopus_states),
        |(flashed_count_total, octopus_states), _| {
            let (flashed_count, new_states) = step_octopus_population(octopus_states);
            (flashed_count_total + flashed_count, new_states)
        },
    );

    flashed_count_total.to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let mut octopus_states = parse_input(input_lines);

    let mut step_count = 0;
    let steps_until_synchronization = loop {
        let (flashed_count, new_states) = step_octopus_population(octopus_states);
        step_count += 1;
        if flashed_count == WIDTH * HEIGHT {
            break step_count;
        }
        octopus_states = new_states;
    };

    steps_until_synchronization.to_string()
}
