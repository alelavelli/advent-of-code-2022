use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::File,
    io::Read,
    time::Instant,
};

use log::info;
use regex::Regex;

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

struct Move {
    qt: i32,
    from: i32,
    to: i32,
}

impl Move {
    fn apply(&self, stacks: &mut HashMap<i32, VecDeque<char>>) {
        for _ in 0..self.qt {
            let elem = stacks.get_mut(&self.from).unwrap().pop_front().unwrap();
            stacks.get_mut(&self.to).unwrap().push_front(elem);
        }
    }

    fn apply_9001(&self, stacks: &mut HashMap<i32, VecDeque<char>>) {
        let queue = stacks.get_mut(&self.from).unwrap();
        let elems = queue.drain(..(self.qt as usize)).collect::<VecDeque<_>>();
        let destination_stack = stacks.get_mut(&self.to).unwrap();
        for elem in elems.into_iter().rev() {
            destination_stack.push_front(elem);
        }
    }
}

fn parse_input(puzzle_input: String) -> (HashMap<i32, VecDeque<char>>, Vec<Move>) {
    let mut split = puzzle_input.split("\n\n");
    let stacks_to_parse = split.next().unwrap();
    let moves_to_parse = split.next().unwrap();

    let mut stacks = HashMap::new();
    for line in stacks_to_parse.lines() {
        for (stack_id, block) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if let Some(crate_name) = block
                .iter()
                .collect::<String>()
                .trim()
                .replace("[", "")
                .replace("]", "")
                .chars()
                .next()
            {
                if !crate_name.is_ascii_digit() {
                    stacks
                        .entry(1 + stack_id as i32)
                        .or_insert(VecDeque::new())
                        .push_back(crate_name);
                }
            }
        }
    }

    let mut moves = Vec::new();
    let re = Regex::new(r"\b\d+\b").unwrap();
    for move_to_parse in moves_to_parse.lines() {
        let matches: Vec<i32> = re
            .find_iter(move_to_parse)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();

        moves.push(Move {
            qt: matches[0],
            from: matches[1],
            to: matches[2],
        });
    }
    (stacks, moves)
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (mut stacks, moves) = parse_input(puzzle_input);

    for move_to_apply in moves {
        move_to_apply.apply(&mut stacks);
    }

    let mut result = String::new();
    for i in 1..=*stacks.keys().max().unwrap() {
        result.push(stacks.get_mut(&i).unwrap().pop_front().unwrap());
    }
    Ok(result)
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (mut stacks, moves) = parse_input(puzzle_input);

    for move_to_apply in moves {
        move_to_apply.apply_9001(&mut stacks);
    }

    let mut result = String::new();
    for i in 1..=*stacks.keys().max().unwrap() {
        result.push(stacks.get_mut(&i).unwrap().pop_front().unwrap());
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_05_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("CMZ".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_05_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("MCD".to_string(), result);

        Ok(())
    }
}
