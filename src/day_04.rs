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

fn build_range(input: &str) -> (i32, i32) {
    let range = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (range[0], range[1])
}

fn is_fully_contained(range: (i32, i32), other: (i32, i32)) -> bool {
    (range.0 >= other.0) & (range.1 <= other.1)
}

fn overlaps(range: (i32, i32), other: (i32, i32)) -> bool {
    !((range.1 < other.0) | (range.0 > other.1))
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut result = 0;
    for pair in puzzle_input.lines() {
        let (first, second) = {
            let mut split = pair.split(',');
            (
                build_range(split.next().unwrap()),
                build_range(split.next().unwrap()),
            )
        };
        if is_fully_contained(first, second) | is_fully_contained(second, first) {
            result += 1;
        }
    }

    Ok(result.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut result = 0;
    for pair in puzzle_input.lines() {
        let (first, second) = {
            let mut split = pair.split(',');
            (
                build_range(split.next().unwrap()),
                build_range(split.next().unwrap()),
            )
        };
        if overlaps(first, second) {
            result += 1;
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
        let mut file = File::open("inputs/day_04_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!(String::from("2"), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_04_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!(String::from("4"), result);

        Ok(())
    }
}
