use std::{error::Error, fs::File, io::Read, time::Instant, vec};

use log::info;

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
    info!("Problem solution is {}", result);
    Ok(())
}

fn parse_input(puzzle_input: String) -> Vec<i8> {
    puzzle_input
        .chars()
        .map(|c| if c == '<' { -1 } else { 1 })
        .collect()
}

/// A Rock is composed of the area that is a vector of
/// u8 representing the occupied bits and the coordinates of the first
/// element of the bottom line
struct Rock {
    area: Vec<u8>,
    heigth: u32,
    rock_type: RockType,
}

#[derive(PartialEq)]
enum RockType {
    Minus,
    Plus,
    ReverseL,
    Pipe,
    Square,
}

fn rock_factory(chamber_width: u8, rock_type: &RockType) -> Rock {
    // shift bits by chamber_width - falling_rock.width - falling_rock.coordinates.0
    match rock_type {
        RockType::Minus => Rock {
            area: vec![15 << (chamber_width - 4 - 2)],
            heigth: 0,
            rock_type: RockType::Minus,
        },
        RockType::Plus => Rock {
            area: vec![
                2 << (chamber_width - 3 - 2),
                7 << (chamber_width - 3 - 2),
                2 << (chamber_width - 3 - 2),
            ],
            heigth: 0,
            rock_type: RockType::Plus,
        },
        RockType::ReverseL => Rock {
            area: vec![
                7 << (chamber_width - 3 - 2),
                1 << (chamber_width - 3 - 2),
                1 << (chamber_width - 3 - 2),
            ],
            heigth: 0,
            rock_type: RockType::ReverseL,
        },
        RockType::Pipe => Rock {
            area: vec![
                1 << (chamber_width - 1 - 2),
                1 << (chamber_width - 1 - 2),
                1 << (chamber_width - 1 - 2),
                1 << (chamber_width - 1 - 2),
            ],
            heigth: 0,
            rock_type: RockType::Pipe,
        },
        RockType::Square => Rock {
            area: vec![3 << (chamber_width - 2 - 2), 3 << (chamber_width - 2 - 2)],
            heigth: 0,
            rock_type: RockType::Square,
        },
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let jet_sequence = parse_input(puzzle_input);
    let mut jet_pattern = jet_sequence.iter().cycle();
    let chamber_width: u8 = 7;

    let rocks = vec![
        RockType::Minus,
        RockType::Plus,
        RockType::ReverseL,
        RockType::Pipe,
        RockType::Square,
    ];
    let mut rock_cycle = rocks.iter().cycle();
    // the chamber is a vector of bitmask with 8 bits representing the chamber width
    // 0 element is bottom and higher elements represent the heght
    let mut chamber: Vec<u8> = Vec::new();
    // add floor which is represented as 1111111
    chamber.push((1 << chamber_width) - 1);

    for _ in 0..2022 {
        let mut falling_rock = rock_factory(chamber_width, rock_cycle.next().unwrap());
        // the rock starts 3 units above the highest rock in the room
        falling_rock.heigth = chamber.len() as u32 + 3;
        loop {
            // get the jet and move the rock
            let &jet = jet_pattern.next().unwrap();
            if jet > 0 {
                let mut can_move = true;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        // check if the rock can move or it hits other rocks or the chamber boundary
                        if (chamber_line & (falling_line >> 1) != 0) | (falling_line & 1 != 0) {
                            can_move = false;
                            break;
                        }
                    } else {
                        // check only if the rock hits the chamber boundary
                        if falling_line & 1 != 0 {
                            can_move = false;
                            break;
                        }
                    }
                }
                if can_move {
                    for falling_line in falling_rock.area.iter_mut() {
                        *falling_line >>= 1;
                    }
                }
            } else {
                let mut can_move = true;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        // check if the rock can move or if it hits other rocks or the chamber boundary
                        if (chamber_line & (falling_line << 1) != 0)
                            | ((falling_line << 1) & (1 << chamber_width) != 0)
                        {
                            can_move = false;
                            break;
                        }
                    } else {
                        // check only if the rock hits the chamber boundary
                        if (falling_line << 1) & (1 << chamber_width) != 0 {
                            can_move = false;
                            break;
                        }
                    }
                }
                if can_move {
                    for falling_line in falling_rock.area.iter_mut() {
                        *falling_line <<= 1;
                    }
                }
            }
            // the rock can go down if the chamber height is lower than the y coordinate
            // of the rock
            if (chamber.len() as u32) < falling_rock.heigth {
                falling_rock.heigth -= 1;
            } else {
                // here we check if there is a rock under the following one otherwise
                // we can go down again

                /*
                for each line of the rock we check if the chamber overlaps with the line
                as it would one step down
                */
                let mut overlapped = false;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth - 1 + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        if chamber_line & falling_line != 0 {
                            // they are overlapped, hence we cannot go down
                            overlapped = true;
                            break;
                        }
                    }
                }
                if overlapped {
                    // the rock cannot go down anymore so we proceed with the loop
                    for (i, falling_line) in falling_rock.area.iter().enumerate() {
                        let chamber_line_id = falling_rock.heigth + i as u32;
                        if let Some(chamber_line) = chamber.get_mut(chamber_line_id as usize) {
                            *chamber_line |= falling_line;
                        } else {
                            chamber.push(*falling_line);
                        }
                    }
                    break;
                } else {
                    falling_rock.heigth -= 1;
                }
            }
        }
    }

    Ok((chamber.len() - 1).to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let jet_sequence = parse_input(puzzle_input);
    let mut jet_pattern = jet_sequence.iter().enumerate().cycle();
    let chamber_width: u8 = 7;

    let rocks = vec![
        RockType::Minus,
        RockType::Plus,
        RockType::ReverseL,
        RockType::Pipe,
        RockType::Square,
    ];
    let mut rock_cycle = rocks.iter().cycle();
    // the chamber is a vector of bitmask with 8 bits representing the chamber width
    // 0 element is bottom and higher elements represent the heght
    let mut chamber: Vec<u8> = Vec::new();
    // add floor which is represented as 1111111
    chamber.push((1 << chamber_width) - 1);

    // Encode the state of the felt rocks and check if it repeats
    // then multiply this height for the remaining iterations
    // the state is the or between K lines of the chamber
    let buffer_size = 10;
    // the state is composed of an encoding ot the rocks in the chamber, the rock that has fallen and the jet id
    let mut chamber_state_history: Vec<(i128, (u128, RockType, usize))> = vec![];

    let max_iterations = 1000000000000_i128;
    let mut iteration_heights: Vec<usize> = Vec::new();
    // this variable is set when a cycle in the falling rocks is found
    let mut state_match_iteration: i128 = 0;

    'rocks_iter: for iteration in 0..max_iterations {
        let mut falling_rock = rock_factory(chamber_width, rock_cycle.next().unwrap());
        // the rock starts 3 units above the highest rock in the room
        falling_rock.heigth = chamber.len() as u32 + 3;
        'falling_loop: loop {
            // get the jet and move the rock
            let (jet_id, &jet) = jet_pattern.next().unwrap();
            if jet > 0 {
                let mut can_move = true;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        // check if the rock can move or it hits other rocks or the chamber boundary
                        if (chamber_line & (falling_line >> 1) != 0) | (falling_line & 1 != 0) {
                            can_move = false;
                            break;
                        }
                    } else {
                        // check only if the rock hits the chamber boundary
                        if falling_line & 1 != 0 {
                            can_move = false;
                            break;
                        }
                    }
                }
                if can_move {
                    for falling_line in falling_rock.area.iter_mut() {
                        *falling_line >>= 1;
                    }
                }
            } else {
                let mut can_move = true;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        // check if the rock can move or if it hits other rocks or the chamber boundary
                        if (chamber_line & (falling_line << 1) != 0)
                            | ((falling_line << 1) & (1 << chamber_width) != 0)
                        {
                            can_move = false;
                            break;
                        }
                    } else {
                        // check only if the rock hits the chamber boundary
                        if (falling_line << 1) & (1 << chamber_width) != 0 {
                            can_move = false;
                            break;
                        }
                    }
                }
                if can_move {
                    for falling_line in falling_rock.area.iter_mut() {
                        *falling_line <<= 1;
                    }
                }
            }
            // the rock can go down if the chamber height is lower than the y coordinate
            // of the rock
            if (chamber.len() as u32) < falling_rock.heigth {
                falling_rock.heigth -= 1;
            } else {
                // here we check if there is a rock under the following one otherwise
                // we can go down again

                /*
                for each line of the rock we check if the chamber overlaps with the line
                as it would one step down
                */
                let mut overlapped = false;
                for (i, falling_line) in falling_rock.area.iter().enumerate() {
                    let chamber_line_id = falling_rock.heigth - 1 + i as u32;
                    if let Some(chamber_line) = chamber.get(chamber_line_id as usize) {
                        if chamber_line & falling_line != 0 {
                            // they are overlapped, hence we cannot go down
                            overlapped = true;
                            break;
                        }
                    }
                }
                if overlapped {
                    // the rock cannot go down anymore so we proceed with the loop
                    for (i, falling_line) in falling_rock.area.iter().enumerate() {
                        let chamber_line_id = falling_rock.heigth + i as u32;
                        if let Some(chamber_line) = chamber.get_mut(chamber_line_id as usize) {
                            *chamber_line |= falling_line;
                        } else {
                            chamber.push(*falling_line);
                        }
                    }
                    // build the chamber state
                    if chamber.len() > buffer_size {
                        let mut chamber_state: u128 = 0;
                        let mut covered_bits: u8 = 0;
                        for i in 0..buffer_size {
                            let mut chamber_line = *chamber.get(chamber.len() - 1 - i).unwrap();
                            chamber_line ^= covered_bits;
                            covered_bits |= chamber_line;
                            chamber_state |= (chamber_line as u128) << (8 * i);
                        }
                        let state_match = chamber_state_history
                            .iter()
                            .filter(|&x| {
                                (x.1 .0 == chamber_state)
                                    & (x.1 .1 == falling_rock.rock_type)
                                    & (x.1 .2 == jet_id)
                            })
                            .collect::<Vec<&(i128, (u128, RockType, usize))>>();
                        if let Some(state_match_value) = state_match.first() {
                            state_match_iteration = state_match_value.0;
                        }
                        if !state_match.is_empty() {
                            chamber_state_history
                                .push((iteration, (chamber_state, falling_rock.rock_type, jet_id)));
                            iteration_heights.push(chamber.len() - 1);
                            break 'rocks_iter;
                        } else {
                            chamber_state_history
                                .push((iteration, (chamber_state, falling_rock.rock_type, jet_id)));
                        }
                    }

                    break 'falling_loop;
                } else {
                    falling_rock.heigth -= 1;
                }
            }
        }
        iteration_heights.push(chamber.len() - 1);
    }

    let &repeated_state = chamber_state_history
        .iter()
        .filter(|x| x.0 == state_match_iteration)
        .collect::<Vec<&(i128, (u128, RockType, usize))>>()
        .first()
        .unwrap();
    let cycle_length = chamber_state_history.last().unwrap().0 - repeated_state.0;

    let iterations_before_cycle = repeated_state.0 - 1;
    let height_before_cycle = *iteration_heights
        .get(iterations_before_cycle as usize)
        .unwrap();

    let cycle_relative_height = iteration_heights.last().unwrap()
        - iteration_heights.get(repeated_state.0 as usize).unwrap();

    let remaining_iterations = max_iterations - iterations_before_cycle;
    let complete_repetitions = remaining_iterations / cycle_length;

    let cycle_total_height = complete_repetitions * cycle_relative_height as i128;

    let iterations_after_cycle = remaining_iterations % cycle_length;

    let partial_cycle_height = iteration_heights
        .get(repeated_state.0 as usize + iterations_after_cycle as usize)
        .unwrap()
        - iteration_heights.get(repeated_state.0 as usize).unwrap();

    let total_height =
        height_before_cycle as i128 + cycle_total_height + partial_cycle_height as i128;
    // soluzione giusta è 1562536022966 quindi si conta + 1 per qualche motivo
    Ok(total_height.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_17_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("3068".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_17_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;

        let result = solve_pt2(puzzle_input)?;

        assert_eq!("1514285714288".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2_actual() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_17.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;

        let result = solve_pt2(puzzle_input)?;

        assert_eq!("1562536022966".to_string(), result);

        Ok(())
    }
}
