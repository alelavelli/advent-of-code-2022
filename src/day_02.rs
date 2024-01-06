use std::{error::Error, fs::File, io::Read, time::Instant, str::FromStr};

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
    info!("Problem solution is {}", result);
    Ok(())
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut total_points = 0;
    for line in puzzle_input.lines() {
        let mut line_split = line.split_whitespace();
        let opponent_play = Play::from_str(line_split.next().unwrap()).unwrap();
        let my_play = Play::from_str(line_split.next().unwrap()).unwrap();
        if my_play == opponent_play {
            total_points += 3;
        } else if my_play > opponent_play {
            total_points += 6;
        }
        total_points += my_play.get_type_point();
    }
    Ok(total_points.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let mut total_points = 0;
    for line in puzzle_input.lines() {
        let mut line_split = line.split_whitespace();
        let opponent_play = Play::from_str(line_split.next().unwrap()).unwrap();
        let match_result = MatchResult::from_str(line_split.next().unwrap()).unwrap();
        let my_play = match_result.get_play_type(&opponent_play.get_type());
        total_points += my_play.get_type_point() + match_result.get_points();
    }
    Ok(total_points.to_string())
}

#[derive(Debug, PartialEq, Eq, EnumString)]
enum MatchResult {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
}

impl MatchResult {
    fn get_play_type(&self, other: &PlayType) -> PlayType {
        match &self {
            MatchResult::Win => {
                match other {
                    PlayType::Rock => PlayType::Paper,
                    PlayType::Paper => PlayType::Scissors,
                    PlayType::Scissors => PlayType::Rock,
                }
            },
            MatchResult::Lose => {
                match other {
                    PlayType::Rock => PlayType::Scissors,
                    PlayType::Paper => PlayType::Rock,
                    PlayType::Scissors => PlayType::Paper,
                }
            },
            MatchResult::Draw => other.clone()
        }
    }

    fn get_points(&self) -> i32 {
        match &self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PlayType {
    Rock,
    Paper,
    Scissors,
}

impl PlayType {
    fn get_type_point(&self) -> i32 {
        match self {
            PlayType::Rock => 1,
            PlayType::Paper => 2,
            PlayType::Scissors => 3
        }
    }
}

#[derive(Debug, EnumString)]
enum Play {
    A,
    B,
    C,
    Y,
    X,
    Z,
}

impl Play {
    fn get_type(&self) -> PlayType {
        match &self {
            Play::A | Play::X => PlayType::Rock,
            Play::B | Play::Y => PlayType::Paper,
            Play::C | Play::Z => PlayType::Scissors,
        }
    }

    fn get_type_point(&self) -> i32 {
        self.get_type().get_type_point()
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.get_type().eq(&other.get_type())
    }
}

impl Eq for Play {
    
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();

        if self_type == other_type {
            std::cmp::Ordering::Equal
        } else if ((self_type == PlayType::Rock) & (other_type == PlayType::Scissors))
            | ((self_type == PlayType::Paper) & (other_type == PlayType::Rock))
            | ((self_type == PlayType::Scissors) & (other_type == PlayType::Paper))
        {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_02_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("15", result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_02_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("12", result);

        Ok(())
    }
}
