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

    fn try_from(energy_level_ascii: u8) -> Result<Self, Self::Error> {
        match energy_level_ascii {
            b'0' => Ok(OctopusState::L0),
            b'1' => Ok(OctopusState::L1),
            b'2' => Ok(OctopusState::L2),
            b'3' => Ok(OctopusState::L3),
            b'4' => Ok(OctopusState::L4),
            b'5' => Ok(OctopusState::L5),
            b'6' => Ok(OctopusState::L6),
            b'7' => Ok(OctopusState::L7),
            b'8' => Ok(OctopusState::L8),
            b'9' => Ok(OctopusState::L9),
            _ => Err("not a valid starting octopus energy level"),
        }
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> [[OctopusState; WIDTH]; HEIGHT] {
    array_init::from_iter(input_lines.map(|line| {
        array_init::from_iter(line.bytes().map(|b| OctopusState::try_from(b).unwrap())).unwrap()
    }))
    .unwrap()
}

fn for_neighbors<F>(array: &mut [[OctopusState; WIDTH]; HEIGHT], x: usize, y: usize, mut f: F)
where
    F: FnMut(&mut OctopusState),
{
    for neighbor_x in x.saturating_sub(1)..=x.saturating_add(1).min(WIDTH - 1) {
        for neighbor_y in y.saturating_sub(1)..=y.saturating_add(1).min(HEIGHT - 1) {
            if neighbor_x == x && neighbor_y == y {
                continue;
            }
            f(&mut array[neighbor_y][neighbor_x]);
        }
    }
}

fn step_octopus_population(
    mut octopus_states: [[OctopusState; WIDTH]; HEIGHT],
) -> (usize, [[OctopusState; WIDTH]; HEIGHT]) {
    let mut new_octopus_states = [[OctopusState::L0; WIDTH]; HEIGHT];

    for (new_octopus_state, &octopus_state) in new_octopus_states
        .iter_mut()
        .flatten()
        .zip(octopus_states.iter().flatten())
    {
        *new_octopus_state = octopus_state.increase_energy();
    }

    octopus_states = new_octopus_states;

    loop {
        for ((x, y), _octopus_state) in octopus_states
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, state)| ((x, y), state))
            })
            .filter(|&(_, octopus_state)| *octopus_state == OctopusState::Charged)
        {
            new_octopus_states[y][x] = OctopusState::Flashed;
            for_neighbors(&mut new_octopus_states, x, y, |neighbor| {
                *neighbor = neighbor.increase_energy();
            });
        }

        if new_octopus_states == octopus_states {
            break;
        }

        octopus_states = new_octopus_states;
    }

    let flashed_count = new_octopus_states
        .iter_mut()
        .flatten()
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
