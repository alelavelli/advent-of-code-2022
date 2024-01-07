use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::Read,
    time::Instant,
};

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
    let min_len = 4;
    let mut window: VecDeque<char> = puzzle_input.chars().take(min_len).collect();
    if window.iter().collect::<HashSet<&char>>().len() == min_len {
        return Ok("4".to_string());
    }

    let mut result = 0;
    for (i, c) in puzzle_input.chars().skip(min_len).enumerate() {
        window.pop_front();
        window.push_back(c);
        if window.iter().collect::<HashSet<&char>>().len() == min_len {
            result = i + min_len + 1;
            break;
        }
    }

    Ok(result.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let min_len = 14;
    let mut window: VecDeque<char> = puzzle_input.chars().take(min_len).collect();
    if window.iter().collect::<HashSet<&char>>().len() == min_len {
        return Ok("4".to_string());
    }

    let mut result = 0;
    for (i, c) in puzzle_input.chars().skip(min_len).enumerate() {
        window.pop_front();
        window.push_back(c);
        if window.iter().collect::<HashSet<&char>>().len() == min_len {
            result = i + min_len + 1;
            break;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_06_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;

        for (seq, solution) in puzzle_input.lines().zip(vec![7, 5, 6, 10, 11]) {
            let result = solve_pt1(seq.to_string())?;
            assert_eq!(solution.to_string(), result);
        }

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_06_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        for (seq, solution) in puzzle_input.lines().zip(vec![19, 23, 23, 29, 26]) {
            let result = solve_pt2(seq.to_string())?;
            assert_eq!(solution.to_string(), result);
        }

        Ok(())
    }
}
