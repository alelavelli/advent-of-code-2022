use std::{error::Error, time::Instant};

use log::info;

use crate::ProblemPart;

pub fn solve(_puzzle_input: &str, part: ProblemPart) -> Result<(), Box<dyn Error>> {
    let puzzle_input = String::new();

    match part {
        ProblemPart::One => {
            info!("Start solving part 1");
            let start = Instant::now();
            solve_pt1(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
        }
    };
    Ok(())
}

fn solve_pt1(_puzzle_input: String) -> Result<(), Box<dyn Error>> {
    info!("Part 1 example execution");
    Ok(())
}

fn solve_pt2(_puzzle_input: String) -> Result<(), Box<dyn Error>> {
    info!("Part 2 example execution");
    Ok(())
}
