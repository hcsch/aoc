use std::error::Error;
use std::fmt::Display;
use std::lazy::SyncLazy;
use std::str::FromStr;

use regex::Regex;

static INSTRUCTION_RE: SyncLazy<Regex> = SyncLazy::new(|| {
    Regex::new(r"^(?P<instruction>acc|jmp|nop) (?P<argument>[+-][0-9]+)$").unwrap()
});

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Accumulate { delta: isize },
    Jump { offset: isize },
    NoOperation { argument: isize },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = INSTRUCTION_RE
            .captures(s)
            .ok_or(format!("Invalid instruction `{}`", s))?;
        let instruction_name = caps.name("instruction").unwrap().as_str();
        let argument: isize = caps.name("argument").unwrap().as_str().parse().unwrap();

        Ok(match instruction_name {
            "acc" => Self::Accumulate { delta: argument },
            "jmp" => Self::Jump { offset: argument },
            "nop" => Self::NoOperation { argument },
            _ => unreachable!(),
        })
    }
}

fn parse_input<I: Iterator<Item = String>>(input_lines: I) -> Vec<Instruction> {
    input_lines.map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
enum ProgramExecutionError {
    EncounteredLoop { accumulator_value: isize },
    InvalidJump,
}

impl Display for ProgramExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ProgramExecutionError {}

fn execute_program(program: &Vec<Instruction>) -> Result<isize, ProgramExecutionError> {
    let mut instruction_visited = vec![false; program.len()];
    let mut program_counter = 0;
    let mut accumulator_value = 0;
    while program_counter < program.len() {
        if instruction_visited[program_counter] {
            Err(ProgramExecutionError::EncounteredLoop { accumulator_value })?;
        }
        let instruction = program[program_counter];
        instruction_visited[program_counter] = true;
        match instruction {
            Instruction::Accumulate { delta } => {
                accumulator_value += delta;
                program_counter += 1;
            }
            Instruction::Jump { offset } => {
                if offset >= 0 {
                    program_counter += offset as usize;
                } else {
                    program_counter = program_counter
                        .checked_sub((-offset) as usize)
                        .ok_or(ProgramExecutionError::InvalidJump)?;
                }
            }
            Instruction::NoOperation { .. } => {
                program_counter += 1;
            }
        }
    }

    if program_counter > program.len() {
        Err(ProgramExecutionError::InvalidJump)?;
    }

    Ok(accumulator_value)
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> isize {
    let program = parse_input(input_lines);

    match execute_program(&program) {
        Err(ProgramExecutionError::EncounteredLoop { accumulator_value }) => accumulator_value,
        _ => panic!("Program does not loop infinitely"),
    }
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> isize {
    let program = parse_input(input_lines);
    let mut modified_program = program.clone();

    let nops_or_jmps_indices = program
        .iter()
        .enumerate()
        .filter_map(|(i, instr)| match instr {
            Instruction::Jump { .. } | Instruction::NoOperation { .. } => Some(i),
            _ => None,
        });
    let mut last_modified_instruction = None;
    for i in nops_or_jmps_indices {
        if let Some(j) = last_modified_instruction {
            modified_program[j] = program[j];
        }

        modified_program[i] = match program[i] {
            Instruction::Jump { offset } => Instruction::NoOperation { argument: offset },
            Instruction::NoOperation { argument } => Instruction::Jump { offset: argument },
            _ => unreachable!(),
        };
        last_modified_instruction = Some(i);

        if let Ok(accumulator_value) = execute_program(&modified_program) {
            return accumulator_value;
        }
    }

    panic!("No modification of one JMP / NOP can fix this program")
}
