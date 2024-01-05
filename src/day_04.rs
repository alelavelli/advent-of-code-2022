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

fn solve_pt1(_puzzle_input: String) -> Result<String, Box<dyn Error>> {
    todo!()
}

fn solve_pt2(_puzzle_input: String) -> Result<String, Box<dyn Error>> {
    todo!()
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let _result = solve_pt1(puzzle_input)?;

        // Add your assertions

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let _result = solve_pt2(puzzle_input)?;

        // Add your assertions

        Ok(())
    }
}
