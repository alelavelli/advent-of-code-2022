use std::{
    collections::HashMap, error::Error, fs::File, io::Read, ops::{Deref, DerefMut}, time::Instant
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
            let duration = start.elapsed().as_millis();
            info!("Solved part 1 in {duration} milli seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_millis();
            info!("Solved part 2 in {duration} milli seconds.");
            result
        }
    };
    info!("Problem solution is {}", result);
    Ok(())
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u64,
    destinations: Vec<String>,
    open: bool,
}
impl Deref for Valve {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.open
    }
}
impl DerefMut for Valve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.open
    }
}
impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let re = Regex::new(
            r"(?<NAME>[A-Z]{2}).*?(?<RATE>\d+).*?valves*\s+(?<DESTINATIONS>(?:[A-Z]{2},\s*)*[A-Z]{2})"
        ).unwrap();
        let capture = re.captures(value).unwrap();
        Valve {
            name: capture
                .name("NAME")
                .map(|x| x.as_str().to_string())
                .unwrap(),
            flow_rate: capture
                .name("RATE")
                .map(|x| x.as_str().parse::<u64>().unwrap())
                .unwrap(),
            destinations: capture
                .name("DESTINATIONS")
                .map(|x| x.as_str().split(", ").map(|x| x.to_string()).collect())
                .unwrap(),
            open: false,
        }
    }
}

fn parse_input(puzzle_input: String) -> Vec<Valve> {
    let mut scan: Vec<Valve> = Vec::new();
    for line in puzzle_input.lines() {
        let valve = Valve::from(line);
        scan.push(valve);
    }
    scan
}

/// from https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn build_adjacency_matrix(
    valves: &Vec<Valve>,
) -> Vec<Vec<u64>> {
    let mut adjacency: Vec<Vec<u64>> = vec![vec![u64::MAX / 2; valves.len()]; valves.len()];
    
    let mut valve_to_id: HashMap<&String, usize> = HashMap::new();
    let mut id_to_valve: HashMap<usize, &String> = HashMap::new();
    for (i, valve) in valves.iter().enumerate() {
        valve_to_id.insert(&valve.name, i);
        id_to_valve.insert(i, &valve.name);
    }

    for (i, valve) in valves.iter().enumerate() {
        adjacency[i][i] = 0;
        for other_valve in valve.destinations.iter() {
            adjacency[i][*valve_to_id.get(other_valve).unwrap()] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let ik = adjacency[i][k];
                let kj = adjacency[k][j];
                let ij = adjacency[i][j];

                if ij > ik + kj {
                    adjacency[i][j] = ik + kj;
                }
            }
        }
    }
    adjacency
}


struct Track {
    current_idx: usize,
    track_mask: u64,
    track_flow: u64,
    remaining_time: u64
}

fn step(
    valves: &Vec<Valve>,
    adjacency: &Vec<Vec<u64>>,
    track: &Track
) -> Option<Vec<Track>> {
    /*
    for the current idx finds all the destinations, compute the time, release
    return all the new tracks as track_mask, track_flow and current_idx
    */
    let mut new_tracks: Vec<Track> = Vec::new();
    let potential_valves = valves
        .iter()
        .enumerate()
        .filter(|(i, v)| {
            // the valve must be closed and with flow rate
            ((1 << i) & track.track_mask == 0) & (v.flow_rate > 0)
        })
        .map(|(i, _)| i);
    for destination_id in potential_valves {
        let time = track
            .remaining_time
            .checked_sub(adjacency[track.current_idx][destination_id])
            .and_then(|t| t.checked_sub(1))
            .unwrap_or(0);
        if time > 0 {
            let released_pressure = valves[destination_id].flow_rate * time;
            new_tracks.push(
                Track {
                track_mask: track.track_mask | (1 << destination_id),
                track_flow: released_pressure + track.track_flow,
                remaining_time: time,
                current_idx: destination_id
            })
        }
    }
    if new_tracks.is_empty() {
        None
    } else {
        Some(new_tracks)
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let valves = parse_input(puzzle_input);
    let adjacency = build_adjacency_matrix(&valves);

    let current_idx = valves.iter().position(|v| v.name == "AA".to_string()).unwrap();
    // 0 means the valve is closed and 1 means that it is open
    let track_mask: u64 = 0;
    let mut active_tracks: Vec<Track> = vec![
        Track {
            current_idx,
            track_flow: 0,
            track_mask,
            remaining_time: 30
        }
    ];
    let mut best_flow = 0;

    while let Some(track) = active_tracks.pop() {
        if let Some(next_tracks) = step(&valves, &adjacency, &track) {
            for next_track in next_tracks {
                if next_track.remaining_time > 0 {
                    active_tracks.push(next_track);
                } else {
                    best_flow = best_flow.max(next_track.track_flow);
                }
            }
        } else {
            best_flow = best_flow.max(track.track_flow);
        }
    }

    Ok(best_flow.to_string())
}

fn solve_pt2(_puzzle_input: String) -> Result<String, Box<dyn Error>> {
    /*
    let valves = parse_input(puzzle_input);
    let (adjacency, valve_to_id, id_to_valve) = build_adjacency_matrix(&valves);

    let mut elf_active_tracks: Vec<Track> = vec![Track::new(
        26,
        valves.clone(),
        &valve_to_id,
        &id_to_valve,
        &adjacency,
    )];
    let mut elf_closed_tracks = Vec::new();

    while let Some(track) = elf_active_tracks.pop() {
        elf_closed_tracks.push(track.clone());
        for next_track in track.step() {
            if next_track.remaining_time > 0 {
                elf_active_tracks.push(next_track);
            } else {
                elf_closed_tracks.push(next_track);
            }
        }
    }

    let mut elephant_active_tracks =  vec![Track::new(
        26,
        valves,
        &valve_to_id,
        &id_to_valve,
        &adjacency,
    )];

    let mut best_mix_flow = 0;
    info!("start processing elephant!");
    while let Some(track) = elephant_active_tracks.pop() {
        for next_track in track.step() {
            best_mix_flow = best_mix_flow.max(elf_closed_tracks.iter().fold(0, |acc, x| { if x.overlaps(&next_track) { acc } else { acc.max(x.released_pressure + next_track.released_pressure) } }));
            /*for elf_track in elf_closed_tracks.iter() {
                if ! elf_track.overlaps(&next_track) {
                    best_mix_flow = best_mix_flow.max(elf_track.released_pressure + next_track.released_pressure);
                }
            }*/
            if next_track.remaining_time > 0 {
                elephant_active_tracks.push(next_track);
            }
        }
    }

    Ok(best_mix_flow.to_string())
     */
    todo!()
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
        let mut file = File::open("inputs/day_16_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("1651".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_16_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("1707".to_string(), result);

        Ok(())
    }
}
