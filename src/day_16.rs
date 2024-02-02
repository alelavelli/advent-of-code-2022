use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    time::Instant,
};

use log::info;
use ndarray::{Array1, Array2, ArrayView2, Axis};
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

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
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
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .unwrap(),
            destinations: capture
                .name("DESTINATIONS")
                .map(|x| x.as_str().split(", ").map(|x| x.to_string()).collect())
                .unwrap(),
            open: false,
        }
    }
}

fn parse_input(puzzle_input: String) -> HashMap<String, Valve> {
    let mut scan: HashMap<String, Valve> = HashMap::new();
    for line in puzzle_input.lines() {
        let valve = Valve::from(line);
        scan.insert(valve.name.clone(), valve);
    }
    scan
}

/// from https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn build_adjacency_matrix(
    valves: &HashMap<String, Valve>,
) -> (Array2<f32>, HashMap<String, usize>, HashMap<usize, String>) {
    let mut adjacency: Array2<f32> = Array2::from_elem((valves.len(), valves.len()), f32::INFINITY);
    let mut valve_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_to_valve: HashMap<usize, String> = HashMap::new();

    for (i, valve_name) in valves.keys().enumerate() {
        valve_to_id.insert(valve_name.clone(), i);
        id_to_valve.insert(i, valve_name.clone());
    }
    for valve_name in valves.keys() {
        let current_valve_id = valve_to_id.get(valve_name).unwrap();
        adjacency[(*current_valve_id, *current_valve_id)] = 0.0;
        for other_valve in valves.get(valve_name).unwrap().destinations.iter() {
            adjacency[(*current_valve_id, *valve_to_id.get(other_valve).unwrap())] = 1.0;
        }
    }

    for k in 0..valves.keys().len() {
        for i in 0..valves.keys().len() {
            for j in 0..valves.keys().len() {
                let ik = adjacency[(i, k)];
                let kj = adjacency[(k, j)];
                let ij = adjacency[(i, j)];

                if ij > ik + kj {
                    adjacency[(i, j)] = ik + kj;
                }
            }
        }
    }

    (adjacency, valve_to_id, id_to_valve)
}

#[derive(Debug, Clone)]
struct Track<'a> {
    // clone of valves because each track modifies the valves
    valves: HashMap<String, Valve>,
    current_valve: String,
    remaining_time: i32,
    released_pressure: u32,
    valve_to_id: &'a HashMap<String, usize>,
    id_to_valve: &'a HashMap<usize, String>,
    adjacency: ArrayView2<'a, f32>,
    path: Vec<String>,
}
impl<'a> Track<'a> {
    fn new(
        valves: HashMap<String, Valve>,
        valve_to_id: &'a HashMap<String, usize>,
        id_to_valve: &'a HashMap<usize, String>,
        adjacency: ArrayView2<'a, f32>,
    ) -> Track<'a> {
        Track {
            valves,
            current_valve: "AA".to_string(),
            remaining_time: 30,
            released_pressure: 0,
            valve_to_id,
            id_to_valve,
            adjacency,
            path: vec!["AA".to_string()],
        }
    }

    /// consume itself and generate new tracks with next opened valves
    fn step(self) -> Vec<Track<'a>> {
        let time = self
            .adjacency
            .index_axis(Axis(0), *self.valve_to_id.get(&self.current_valve).unwrap())
            .mapv(|x| self.remaining_time as f32 - x - 1.0);
        //debug!("TIME vector is {:?}", time);
        let flow = (0..self.valves.len())
            .map(|i| {
                let v = self.id_to_valve.get(&i).unwrap();
                self.valves.get(v).unwrap().flow_rate as f32
            })
            .collect::<Array1<f32>>();
        //debug!("FLOW vector is {:?}", flow);
        let open = (0..self.valves.len())
            .map(|i| {
                let v = self.id_to_valve.get(&i).unwrap();
                if self.valves.get(v).unwrap().open {
                    0.0
                } else {
                    1.0
                }
            })
            .collect::<Array1<f32>>();
        //debug!("OPEN vector is {:?}", open);
        let release = time.clone() * flow * open;

        let mut new_tracks: Vec<Track> = Vec::new();
        for (destination_id, released_pressure) in release
            .to_vec()
            .iter()
            .enumerate()
            .filter(|(_, &r)| r > 0.0)
        {
            let mut new_valves = self.valves.clone();
            let destination_name = self.id_to_valve.get(&destination_id).unwrap();
            new_valves
                .entry(destination_name.clone())
                .and_modify(|x| x.open = true);

            let mut new_path = self.path.clone();
            new_path.push(destination_name.clone());

            new_tracks.push(Track {
                valves: new_valves,
                current_valve: destination_name.clone(),
                remaining_time: time[destination_id] as i32,
                released_pressure: self.released_pressure + *released_pressure as u32,
                valve_to_id: self.valve_to_id,
                id_to_valve: self.id_to_valve,
                adjacency: self.adjacency,
                path: new_path,
            });
        }
        if new_tracks.is_empty() {
            vec![Track {
                valves: self.valves,
                current_valve: self.current_valve,
                remaining_time: 0,
                released_pressure: self.released_pressure,
                valve_to_id: self.valve_to_id,
                id_to_valve: self.id_to_valve,
                adjacency: self.adjacency,
                path: self.path,
            }]
        } else {
            new_tracks
        }
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let valves = parse_input(puzzle_input);
    let (adjacency, valve_to_id, id_to_valve) = build_adjacency_matrix(&valves);

    let mut active_tracks: Vec<Track> = vec![Track::new(
        valves,
        &valve_to_id,
        &id_to_valve,
        adjacency.view(),
    )];
    let mut terminated_tracks: Vec<Track> = Vec::new();

    while let Some(track) = active_tracks.pop() {
        for next_track in track.step() {
            if next_track.remaining_time > 0 {
                active_tracks.push(next_track);
            } else {
                terminated_tracks.push(next_track);
            }
        }
    }
    let best_track = terminated_tracks
        .iter()
        .max_by(|a, b| a.released_pressure.cmp(&b.released_pressure))
        .unwrap();
    Ok(best_track.released_pressure.to_string())
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
        let mut file = File::open("inputs/")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let _result = solve_pt2(puzzle_input)?;

        // Add your assertions

        Ok(())
    }
}
