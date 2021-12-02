use std::str::FromStr;

#[derive(Clone, Copy)]
enum SubmarineCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for SubmarineCommand {
    type Err = &'static str;

    fn from_str(cmd_str: &str) -> Result<Self, <Self as FromStr>::Err> {
        if let Some(units) = cmd_str.strip_prefix("forward ") {
            return Ok(Self::Forward(
                units
                    .parse()
                    .map_err(|_| "expected unsigned integer after `forward`")?,
            ));
        }
        if let Some(units) = cmd_str.strip_prefix("down ") {
            return Ok(Self::Down(
                units
                    .parse()
                    .map_err(|_| "expected unsigned integer after `down`")?,
            ));
        }
        if let Some(units) = cmd_str.strip_prefix("up ") {
            return Ok(Self::Up(
                units
                    .parse()
                    .map_err(|_| "expected unsigned integer after `up`")?,
            ));
        }

        return Err("expected one of `forward <units>`, `down <units>`, `up <units>`");
    }
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> u32 {
    let depths: Vec<SubmarineCommand> = input_lines
        .map(|l| {
            l.parse()
                .expect("Input was not solely lines of unsigned integers")
        })
        .collect();

    let (final_depth, final_horizontal_pos) =
        depths
            .iter()
            .copied()
            .fold((0, 0), |(depth, horizontal_pos), cmd| match cmd {
                SubmarineCommand::Forward(units) => (depth, horizontal_pos + units),
                SubmarineCommand::Down(units) => (depth + units, horizontal_pos),
                SubmarineCommand::Up(units) => (depth - units, horizontal_pos),
            });

    let solution = final_depth * final_horizontal_pos;

    solution
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> u32 {
    let depths: Vec<SubmarineCommand> = input_lines
        .map(|l| {
            l.parse()
                .expect("Input was not solely lines of unsigned integers")
        })
        .collect();

    let (final_depth, final_horizontal_pos, _final_aim) =
        depths
            .iter()
            .copied()
            .fold((0, 0, 0), |(depth, horizontal_pos, aim), cmd| match cmd {
                SubmarineCommand::Forward(units) => {
                    (depth + aim * units, horizontal_pos + units, aim)
                }
                SubmarineCommand::Down(units) => (depth, horizontal_pos, aim + units),
                SubmarineCommand::Up(units) => (depth, horizontal_pos, aim - units),
            });

    let solution = final_depth * final_horizontal_pos;

    solution
}
