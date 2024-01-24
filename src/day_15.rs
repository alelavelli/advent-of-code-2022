use std::{collections::HashSet, error::Error, fs::File, io::Read, time::Instant};

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
            let result = solve_pt1(puzzle_input, 2000000)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input, 4000000)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
            result
        }
    };
    info!("Problem solution is {}", result);
    Ok(())
}

fn manhattan_distance(left: &(i32, i32), right: &(i32, i32)) -> i32 {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

/// returns the upper and lower bounds for x
fn inner_points(sensor: &(i32, i32, i32), y: i32) -> Option<(i32, i32)> {
    /*
    |sx - x| + (sy - y) <= r

    if x > 0 {
        x >= - r + dy + sx
    }

    if x < 0 {
        x <= + r - dy + sx
    }
    */
    let dy = (sensor.1 - y).abs();

    //  x >= - r + dy + sx
    let xge = -sensor.2 + dy + sensor.0;

    // x <= + r - dy + sx
    let xle = sensor.2 - dy + sensor.0;

    if xge > xle {
        None
    } else {
        Some((xge, xle))
    }
}

type Sensors = Vec<(i32, i32, i32)>;
type Beacons = HashSet<(i32, i32)>;

fn parse_input(puzzle_input: String) -> (Sensors, Beacons) {
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();
    let re = Regex::new(r"x=(?P<x>-?\d+), y=(?P<y>-?\d+)").unwrap();
    for line in puzzle_input.lines() {
        let mut re_iter = re.captures_iter(line);

        let sensor_capture = re_iter.next().unwrap();
        let beacon_capture = re_iter.next().unwrap();

        let sensor = (
            sensor_capture
                .name("x")
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .unwrap(),
            sensor_capture
                .name("y")
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .unwrap(),
        );

        let beacon = (
            beacon_capture
                .name("x")
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .unwrap(),
            beacon_capture
                .name("y")
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .unwrap(),
        );

        let distance = manhattan_distance(&sensor, &beacon);
        beacons.insert(beacon);
        sensors.push((sensor.0, sensor.1, distance));
    }

    (sensors, beacons)
}

fn overlaps(left: &(i32, i32), right: &(i32, i32)) -> bool {
    (left.0 <= right.1) && (right.0 <= left.1)
}

fn solve_pt1(puzzle_input: String, y: i32) -> Result<String, Box<dyn Error>> {
    let (sensors, beacons) = parse_input(puzzle_input);
    let mut bounds = sensors
        .iter()
        .filter_map(|s| inner_points(s, y))
        .collect::<Vec<(i32, i32)>>();

    bounds.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ranges: Vec<(i32, i32)> = vec![*bounds.first().unwrap()];
    for bound in bounds.iter().skip(1) {
        let last_range = ranges.last_mut().unwrap();
        if overlaps(last_range, bound) {
            last_range.0 = last_range.0.min(bound.0);
            last_range.1 = last_range.1.max(bound.1);
        } else {
            ranges.push(*bound);
        }
    }

    let mut contained_beacons = 0;
    let y_beacons: Vec<&(i32, i32)> = beacons
        .iter()
        .filter(|e| e.1 == y)
        .collect::<Vec<&(i32, i32)>>();
    for range in ranges {
        let mut range_len = range.1 - range.0 + 1;
        for beacon in y_beacons.iter() {
            if overlaps(&range, beacon) {
                range_len -= 1;
            }
            contained_beacons += range_len;
        }
    }

    Ok(contained_beacons.to_string())
}

fn solve_pt2(puzzle_input: String, max_bound: i32) -> Result<String, Box<dyn Error>> {
    let (sensors, _) = parse_input(puzzle_input);

    for y in 0..=max_bound {
        let mut bounds = sensors
            .iter()
            .filter_map(|s| inner_points(s, y))
            .collect::<Vec<(i32, i32)>>();

        bounds.sort_by(|a, b| a.0.cmp(&b.0));
        let mut first = *bounds.first().unwrap();
        first.0 = first.0.max(0);
        first.1 = first.1.min(max_bound);
        let mut ranges: Vec<(i32, i32)> = vec![first];
        for bound in bounds.iter().skip(1) {
            let last_range = ranges.last_mut().unwrap();
            if overlaps(last_range, bound) {
                last_range.0 = last_range.0.min(bound.0).max(0);
                last_range.1 = last_range.1.max(bound.1).min(max_bound);
            } else {
                ranges.push(*bound);
            }
        }

        let mut occupied_slots = 0;
        for range in ranges.iter() {
            occupied_slots += range.1 - range.0 + 1;
        }
        if occupied_slots == max_bound {
            // find if the x is the left point, the right point or between the two ranges
            let x: u128 = if ranges.len() == 2 {
                (ranges.first().unwrap().1 + 1) as u128
            } else if ranges.first().unwrap().0 == 0 {
                max_bound as u128
            } else {
                0
            };
            let result: u128 = x * 4000000 + y as u128;
            return Ok(result.to_string());
        }
    }
    Ok("mmm".to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_15_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input, 10)?;

        assert_eq!("26".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_15_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input, 20)?;

        assert_eq!("56000011", result);

        Ok(())
    }
}
