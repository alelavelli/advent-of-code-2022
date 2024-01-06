use std::{error::Error, fs::File, io::Read, time::Instant};

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

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut max_calories = 0;

    let mut current_calories = 0;
    for line in puzzle_input.lines() {
        if line.len() == 0 {
            max_calories = max_calories.max(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    Ok(max_calories.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut calories: Vec<i32> = Vec::new();

    let mut current_calories = 0;
    for block in puzzle_input.split("\n\n") {
        for line in block.lines() {
            current_calories += line.parse::<i32>().unwrap();
        }
        calories.push(current_calories);
        current_calories = 0;
    }
    calories.sort();
    calories.reverse();
    Ok(calories.iter().take(3).sum::<i32>().to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_01_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!(String::from("24000"), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_01_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!(String::from("45000"), result);

        Ok(())
    }
}
