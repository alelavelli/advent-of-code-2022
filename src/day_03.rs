use std::{collections::HashSet, error::Error, fs::File, io::Read, time::Instant};

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

const LOWER_OFFSET: u8 = 'a' as u8;
const HIGHER_OFFSET: u8 = 'A' as u8;

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut priority_sum: i32 = 0;
    for line in puzzle_input.lines() {
        let first_compartment = line.chars().take(line.len() / 2).collect::<HashSet<char>>();
        let second_compartment = line.chars().skip(line.len() / 2).collect::<HashSet<char>>();
        let item = first_compartment
            .intersection(&second_compartment)
            .collect::<Vec<&char>>()[0];
        let offset = if item.is_ascii_lowercase() {
            LOWER_OFFSET - 1
        } else {
            HIGHER_OFFSET - 27
        };
        priority_sum += (*item as u8 - offset) as i32;
    }
    Ok(priority_sum.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut priority_sum: i32 = 0;
    for group in puzzle_input.lines().collect::<Vec<&str>>().chunks(3) {
        let badge = group
            .iter()
            .map(|&x| x.chars().collect::<HashSet<char>>())
            .reduce(|a, b| a.intersection(&b).map(|x| *x).collect())
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let offset = if badge.is_ascii_lowercase() {
            LOWER_OFFSET - 1
        } else {
            HIGHER_OFFSET - 27
        };
        priority_sum += (badge as u8 - offset) as i32;
    }
    Ok(priority_sum.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_03_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!(String::from("157"), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_03_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!(String::from("70"), result);

        Ok(())
    }
}
