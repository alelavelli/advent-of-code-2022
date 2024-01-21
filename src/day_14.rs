use std::{
    collections::{HashMap, HashSet},
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

fn parse_pair(pair: &str) -> (u32, u32) {
    let mut elems = pair.split(',');
    let first = elems.next().unwrap().parse().unwrap();
    let second = elems.next().unwrap().parse().unwrap();
    // row and column are in the reverse order
    (second, first)
}

type Scan = HashSet<(u32, u32)>;
type Floor = HashMap<u32, Vec<u32>>;

fn parse_input(puzzle_input: String) -> (Scan, Floor) {
    // for each coordinate contains if there is a rock
    let mut scan: HashSet<(u32, u32)> = HashSet::new();
    // for each column contains the highest occupied row
    let mut floor: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in puzzle_input.lines() {
        let mut line_iter = line.split(" -> ");
        let mut prev_step: (u32, u32) = parse_pair(line_iter.next().unwrap());

        for raw_step in line_iter {
            let step = parse_pair(raw_step);
            floor
                .entry(step.1)
                .and_modify(|e| {
                    e.push(step.0);
                })
                .or_insert(vec![step.0]);

            let from_r = prev_step.0.min(step.0);
            let to_r = prev_step.0.max(step.0);
            let from_c = prev_step.1.min(step.1);
            let to_c = prev_step.1.max(step.1);

            for r in from_r..=to_r {
                for c in from_c..=to_c {
                    scan.insert((r, c));

                    floor
                        .entry(c)
                        .and_modify(|e| {
                            e.push(r);
                        })
                        .or_insert(vec![r]);
                }
            }
            prev_step = step;
        }
    }
    (scan, floor)
}

fn fall(
    scan: &HashSet<(u32, u32)>,
    floor: &HashMap<u32, Vec<u32>>,
    starting_position: &(u32, u32),
) -> Option<(u32, u32)> {
    if starting_position.1 == 0 {
        // since we reached the extreme left the sand unit will fall forever
        None
    } else if let Some(Some(&center)) = floor
        .get(&starting_position.1)
        .map(|centers| centers.iter().filter(|&&c| c > starting_position.0).min())
    {
        if !scan.contains(&(center, starting_position.1 - 1)) {
            // the left is empty so the sand unit goes there and then we check the fall
            fall(scan, floor, &(center, starting_position.1 - 1))
        } else if !scan.contains(&(center, starting_position.1 + 1)) {
            // the right is empty so the sand unit goes there and then we check the fall
            fall(scan, floor, &(center, starting_position.1 + 1))
        } else {
            Some((center - 1, starting_position.1))
        }
    } else {
        // if there is no floor then the sand will fall forever
        None
    }
}

fn fall_with_floor(
    scan: &HashSet<(u32, u32)>,
    floor: &HashMap<u32, Vec<u32>>,
    starting_position: &(u32, u32),
    floor_row: u32,
) -> Option<(u32, u32)> {
    if starting_position.1 == 0 {
        // since we reached the extreme left the sand unit will fall forever
        None
    } else if let Some(Some(&center)) = floor
        .get(&starting_position.1)
        .map(|centers| centers.iter().filter(|&&c| c > starting_position.0).min())
    {
        if !scan.contains(&(center, starting_position.1 - 1)) {
            // the left is empty so the sand unit goes there and then we check the fall
            fall_with_floor(scan, floor, &(center, starting_position.1 - 1), floor_row)
        } else if !scan.contains(&(center, starting_position.1 + 1)) {
            // the right is empty so the sand unit goes there and then we check the fall
            fall_with_floor(scan, floor, &(center, starting_position.1 + 1), floor_row)
        } else {
            Some((center - 1, starting_position.1))
        }
    } else {
        // if there is no floor we hit the actual floor
        Some((floor_row - 1, starting_position.1))
    }
}

fn _print_scan(rocks_scan: &HashSet<(u32, u32)>, full_scan: &HashSet<(u32, u32)>) {
    println!();
    for r in 0..=full_scan.iter().map(|x| x.0).max().unwrap() {
        print!("{r}: ");
        for c in full_scan.iter().map(|x| x.1).min().unwrap()
            ..=full_scan.iter().map(|x| x.1).max().unwrap()
        {
            if rocks_scan.contains(&(r, c)) {
                print!("#");
            } else if full_scan.contains(&(r, c)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (mut scan, mut floor) = parse_input(puzzle_input);
    let mut sands_unit = 0;

    let source_col = 500;
    let source_row = 0;

    loop {
        let final_position = fall(&scan, &floor, &(source_row, source_col));
        if let Some(final_position) = final_position {
            floor.entry(final_position.1).and_modify(|x| {
                x.push(final_position.0);
            });
            scan.insert(final_position);
            sands_unit += 1;
        } else {
            break;
        }
    }
    Ok(sands_unit.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (mut scan, mut floor) = parse_input(puzzle_input);
    //print_scan(&rock_scan, &scan);
    let mut sands_unit = 0;
    let floor_row = scan.iter().map(|x| x.0).max().unwrap() + 2;

    let source_col = 500;
    let source_row = 0;

    loop {
        let final_position = fall_with_floor(&scan, &floor, &(source_row, source_col), floor_row);

        if let Some(final_position) = final_position {
            floor
                .entry(final_position.1)
                .and_modify(|x| {
                    x.push(final_position.0);
                })
                .or_insert(vec![final_position.0]);
            scan.insert(final_position);
            sands_unit += 1;
            //print_scan(&rock_scan, &scan);
            if final_position == (source_row, source_col) {
                break;
            }
        } else {
            break;
        }
    }
    Ok(sands_unit.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_14_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("24".to_string(), result);
        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_14_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("93".to_string(), result);

        Ok(())
    }
}
