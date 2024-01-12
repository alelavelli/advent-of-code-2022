use std::{collections::HashMap, error::Error, fs::File, io::Read, str::FromStr, time::Instant};

use log::info;
use strum_macros::EnumString;

use crate::ProblemPart;

pub fn solve(puzzle_input: &str, part: ProblemPart) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(puzzle_input)?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let result = match part {
        ProblemPart::One => {
            info!("Start solving part 1");
            let start = Instant::now();
            let result = solve_pt1(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
            result
        }
    };
    info!("Problem solution is \n{}", result);
    Ok(())
}

#[derive(EnumString)]
enum Instruction {
    #[strum(ascii_case_insensitive)]
    Noop,
    #[strum(serialize = "addx")]
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

struct Program {
    initial_state: i32,
    instructions: Vec<Instruction>,
    /// maps the nth cycle to the program state
    cycle_state_map: HashMap<i32, i32>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        let initial_cycle = 1;
        let initial_state = 1;

        let cycle_state_map = instructions
            .iter()
            .scan((initial_cycle, initial_state), |acc, x| {
                let cycle = acc.0 + x.cycles();
                let state = acc.1 + {
                    if let Instruction::Addx(value) = x {
                        value
                    } else {
                        &0
                    }
                };
                *acc = (cycle, state);
                Some(*acc)
            })
            .collect();

        Program {
            initial_state,
            instructions,
            cycle_state_map,
        }
    }

    /// without executing the program returns that state the program has
    /// at the given cycle
    ///
    /// None is returned if for that cycle the program terminated its execution
    fn strength_at_nth_cycle(&self, cycle: i32) -> Option<i32> {
        if cycle > self.program_len() {
            None
        } else {
            // we find the index of the instruction under execution
            if let Some(state) = self.cycle_state_map.get(&cycle) {
                Some(*state * cycle)
            } else {
                self.cycle_state_map
                    .get(&(cycle - 1))
                    .map(|state| *state * cycle)
                    .or(Some(self.initial_state))
            }
        }
    }

    /// without executing the program returns that state the program has
    /// at the given cycle
    ///
    /// None is returned if for that cycle the program terminated its execution
    fn state_at_nth_cycle(&self, cycle: i32) -> Option<i32> {
        if cycle > self.program_len() {
            None
        } else {
            // we find the index of the instruction under execution
            if let Some(state) = self.cycle_state_map.get(&cycle) {
                Some(*state)
            } else {
                self.cycle_state_map
                    .get(&(cycle - 1))
                    .copied()
                    .or(Some(self.initial_state))
            }
        }
    }

    /// returns the length in cycles of the program
    fn program_len(&self) -> i32 {
        self.instructions.iter().map(|x| x.cycles()).sum()
    }
}

fn parse_input(puzzle_input: String) -> Program {
    let mut instructions = Vec::new();
    for line in puzzle_input.lines() {
        let instruction_name = line.split_whitespace().next().unwrap();
        let mut instruction = Instruction::from_str(instruction_name).unwrap();
        if let Instruction::Addx(ref mut value) = instruction {
            *value = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
        }
        instructions.push(instruction);
    }
    Program::new(instructions)
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let program = parse_input(puzzle_input);
    let mut result = 0;
    // per qualche motivo al ciclo 220 lo stato è 19 e non 18
    let mut cycle = 20;
    while program.program_len() >= cycle {
        result += program.strength_at_nth_cycle(cycle).unwrap();
        cycle += 40;
    }

    Ok(result.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let program = parse_input(puzzle_input);
    let mut result = String::new();
    // per qualche motivo al ciclo 220 lo stato è 19 e non 18
    for i in 0..240 {
        let sprite_mid_position = program.state_at_nth_cycle(i + 1).unwrap();
        if (sprite_mid_position - 1 <= i % 40) & (i % 40 <= sprite_mid_position + 1) {
            result.push('#');
        } else {
            result.push('.');
        }
        if ((i + 1) % 40 == 0) & (i + 1 > 0) {
            result.push('\n');
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_10_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("13140".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_10_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;
        let right_result = String::from("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n");
        println!("RESULT\n{result}");
        println!("\n\nRIGHT RESULT\n{right_result}");
        assert_eq!(right_result, result);

        Ok(())
    }
}
