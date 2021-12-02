use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::str::FromStr;

use cgmath::{Basis2, Deg, InnerSpace, Point2, Rad, Rotation, Rotation2, Vector2};

#[derive(Debug, Clone, Copy)]
struct Ship {
    /// An angle in radians, where 0 is east. A positive value implies a CCW-rotation.
    facing_direction: Rad<f64>,
    position: Point2<f64>,
}

impl Ship {
    pub fn exec_instruction(&mut self, nav_instruction: NavInstruction) {
        match nav_instruction {
            NavInstruction::Move {
                direction,
                distance,
            } => self.move_in_direction(distance, direction),
            NavInstruction::Turn { direction, angle } => {
                self.facing_direction += match direction {
                    TurnDirection::Left => angle,
                    TurnDirection::Right => -angle,
                }
            }
        }
    }

    fn move_in_direction(&mut self, distance: f64, direction: MoveDirection) {
        let move_dir_radians = match direction {
            MoveDirection::North => Deg(90f64).into(),
            MoveDirection::East => Deg(0f64).into(),
            MoveDirection::South => Deg(-90f64).into(),
            MoveDirection::West => Deg(180f64).into(),
            MoveDirection::Forward => self.facing_direction,
        };

        self.position = self.position
            + Basis2::from_angle(move_dir_radians).rotate_vector(Vector2::from([distance, 0.0]));
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            facing_direction: Rad(0.0),
            position: Point2::from([0.0, 0.0]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveDirection {
    North,
    East,
    South,
    West,
    Forward,
}

impl TryFrom<u8> for MoveDirection {
    type Error = &'static str;

    fn try_from(direction_char_byte: u8) -> Result<Self, Self::Error> {
        Ok(match direction_char_byte {
            b'N' => MoveDirection::North,
            b'E' => MoveDirection::East,
            b'S' => MoveDirection::South,
            b'W' => MoveDirection::West,
            b'F' => MoveDirection::Forward,
            _ => Err("Invalid move direction character byte")?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnDirection {
    Left,
    Right,
}

impl TryFrom<u8> for TurnDirection {
    type Error = &'static str;

    fn try_from(direction_char_byte: u8) -> Result<Self, Self::Error> {
        Ok(match direction_char_byte {
            b'L' => TurnDirection::Left,
            b'R' => TurnDirection::Right,
            _ => Err("Invalid turn direction character byte")?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NavInstruction {
    Move {
        direction: MoveDirection,
        distance: f64,
    },
    Turn {
        direction: TurnDirection,
        /// An angle in radians, where 0 is east. A positive value implies a CCW-rotation.
        angle: Rad<f64>,
    },
}

impl FromStr for NavInstruction {
    type Err = &'static str;

    fn from_str(nav_instruction_str: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        Ok(
            if let Ok(move_dir) = nav_instruction_str.as_bytes()[0].try_into() {
                Self::Move {
                    direction: move_dir,
                    distance: nav_instruction_str[1..]
                        .parse()
                        .map_err(|_| "Invalid move instruction str")?,
                }
            } else if let Ok(turn_dir) = nav_instruction_str.as_bytes()[0].try_into() {
                Self::Turn {
                    direction: turn_dir,
                    angle: nav_instruction_str[1..]
                        .parse::<f64>()
                        .map(Deg)
                        .map(Rad::from)
                        .map_err(|_| "Invalid turn instruction str")?,
                }
            } else {
                Err("Invalid navigation instruction str")?
            },
        )
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> impl Iterator<Item = NavInstruction> {
    input_lines.map(|line| line.parse().unwrap())
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let nav_instructions = parse_input(input_lines);

    let final_pos = nav_instructions
        .fold(Ship::default(), |mut ship, nav_instruction| {
            ship.exec_instruction(nav_instruction);
            ship
        })
        .position;
    (final_pos.x.abs() + final_pos.y.abs()).to_string()
}

#[derive(Debug, Clone, Copy)]
struct ShipV2 {
    /// An angle in radians, where 0 is east. A positive value implies a CCW-rotation.
    position: Point2<f64>,
    waypoint_rel_pos: Vector2<f64>,
}

impl ShipV2 {
    pub fn exec_instruction(&mut self, nav_instruction: NavInstruction) {
        match nav_instruction {
            NavInstruction::Move {
                direction: MoveDirection::Forward,
                distance,
            } => {
                self.position = self.position
                    + self.waypoint_rel_pos / self.waypoint_rel_pos.magnitude2() * distance
            }
            NavInstruction::Move {
                direction,
                distance,
            } => self.move_waypoint(distance, direction),
            NavInstruction::Turn { direction, angle } => {
                self.rotate_waypoint_around_ship(match direction {
                    TurnDirection::Left => angle,
                    TurnDirection::Right => -angle,
                })
            }
        }
    }

    fn rotate_waypoint_around_ship<R: Into<Rad<f64>>>(&mut self, angle_rad: R) {
        self.waypoint_rel_pos = Basis2::from_angle(angle_rad).rotate_vector(self.waypoint_rel_pos);
    }

    fn move_waypoint(&mut self, distance: f64, direction: MoveDirection) {
        self.waypoint_rel_pos += match direction {
            MoveDirection::North => Vector2::from([0.0, distance]),
            MoveDirection::East => Vector2::from([distance, 0.0]),
            MoveDirection::South => Vector2::from([0.0, -distance]),
            MoveDirection::West => Vector2::from([-distance, 0.0]),
            MoveDirection::Forward => panic!("Cannot move waypoint forward"),
        };
    }
}

impl Default for ShipV2 {
    fn default() -> Self {
        Self {
            position: Point2::from([0.0, 0.0]),
            waypoint_rel_pos: Vector2::from([10.0, 1.0]),
        }
    }
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let nav_instructions = parse_input(input_lines);

    let final_pos = nav_instructions
        .fold(ShipV2::default(), |mut ship, nav_instruction| {
            ship.exec_instruction(nav_instruction);
            ship
        })
        .position;
    (final_pos.x.abs() + final_pos.y.abs()).to_string()
}
