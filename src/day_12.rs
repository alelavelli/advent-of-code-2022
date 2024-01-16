use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    f32::INFINITY,
    fs::File,
    io::Read,
    time::Instant,
};

use log::info;
use ndarray::{Array2, ArrayView2};

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

fn parse_input(puzzle_input: String) -> (Array2<i32>, (usize, usize), (usize, usize)) {
    let lines = puzzle_input.lines().collect::<Vec<&str>>();

    let mut heightmap: Array2<i32> = Array2::zeros((lines.len(), lines[0].len()));
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (r, &line) in lines.iter().enumerate() {
        for (c, elem) in line.chars().enumerate() {
            let h = match elem {
                'S' => {
                    start = (r, c);
                    'a' as i32
                }
                'E' => {
                    end = (r, c);
                    'z' as i32
                }
                _ => elem as i32,
            };
            heightmap[(r, c)] = h;
        }
    }
    (heightmap, start, end)
}

fn find_neighbors(node: &(usize, usize), heightmap: ArrayView2<i32>) -> Vec<(usize, usize)> {
    // look at neighbors and keep nodes with difference of value at most 1
    let mut neighbors = Vec::new();

    // up
    if node.0 >= 1 && heightmap[*node] + 1 >= heightmap[(node.0 - 1, node.1)] {
        neighbors.push((node.0 - 1, node.1));
    }

    // down
    if node.0 < heightmap.shape()[0] - 1 && heightmap[*node] + 1 >= heightmap[(node.0 + 1, node.1)]
    {
        neighbors.push((node.0 + 1, node.1));
    }

    // left
    if node.1 >= 1 && heightmap[*node] + 1 >= heightmap[(node.0, node.1 - 1)] {
        neighbors.push((node.0, node.1 - 1));
    }

    // right
    if node.1 < heightmap.shape()[1] - 1 && heightmap[*node] + 1 >= heightmap[(node.0, node.1 + 1)]
    {
        neighbors.push((node.0, node.1 + 1));
    }

    neighbors
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (heightmap, start, end) = parse_input(puzzle_input);
    let mut unvisited_set: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited_set: HashSet<(usize, usize)> = HashSet::new();
    let mut tentative_distance: HashMap<(usize, usize), f32> = HashMap::new();
    let mut current_node: (usize, usize) = start;
    tentative_distance.insert(current_node, 0.0);
    unvisited_set.push_back(start);
    let mut destination_node_marked = false;

    while !unvisited_set.is_empty() & !destination_node_marked {
        current_node = unvisited_set.pop_front().unwrap();
        for neighbor_node in find_neighbors(&current_node, heightmap.view()) {
            // neighbor distance is always 1 because only one step of one is allowed
            let neighbor_distance = 1.0;
            let distance = neighbor_distance + tentative_distance.get(&current_node).unwrap();
            let current_neighbor_distance =
                tentative_distance.entry(neighbor_node).or_insert(INFINITY);
            if *current_neighbor_distance > distance {
                *current_neighbor_distance = distance;
            }

            if !visited_set.contains(&neighbor_node) & !unvisited_set.contains(&neighbor_node) {
                unvisited_set.push_back(neighbor_node);
            }

            if end == neighbor_node {
                destination_node_marked = true;
            }
        }
        visited_set.insert(current_node);
    }

    Ok(tentative_distance.get(&end).unwrap().to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let (heightmap, start, end) = parse_input(puzzle_input);

    let mut candiates_starts: Vec<(usize, usize)> = vec![start];
    for r in 0..heightmap.shape()[0] {
        for c in 0..heightmap.shape()[1] {
            if heightmap[(r, c)] == heightmap[start] {
                candiates_starts.push((r, c));
            }
        }
    }

    let mut minimum_distance = INFINITY;

    for start in candiates_starts {
        info!("Processing {:?}", start);

        let mut unvisited_set: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited_set: HashSet<(usize, usize)> = HashSet::new();
        let mut tentative_distance: HashMap<(usize, usize), f32> = HashMap::new();
        let mut current_node: (usize, usize) = start;
        tentative_distance.insert(current_node, 0.0);
        unvisited_set.push_back(start);
        let mut destination_node_marked = false;

        while !unvisited_set.is_empty() & !destination_node_marked {
            current_node = unvisited_set.pop_front().unwrap();
            for neighbor_node in find_neighbors(&current_node, heightmap.view()) {
                // neighbor distance is always 1 because only one step of one is allowed
                let neighbor_distance = 1.0;
                let distance = neighbor_distance + tentative_distance.get(&current_node).unwrap();
                let current_neighbor_distance =
                    tentative_distance.entry(neighbor_node).or_insert(INFINITY);
                if *current_neighbor_distance > distance {
                    *current_neighbor_distance = distance;
                }

                if !visited_set.contains(&neighbor_node) & !unvisited_set.contains(&neighbor_node) {
                    unvisited_set.push_back(neighbor_node);
                }

                if end == neighbor_node {
                    destination_node_marked = true;
                }
            }
            visited_set.insert(current_node);
        }

        if *tentative_distance.get(&end).unwrap_or(&INFINITY) < minimum_distance {
            minimum_distance = *tentative_distance.get(&end).unwrap();
        }
    }
    Ok(minimum_distance.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_12_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("31".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_12_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("29".to_string(), result);

        Ok(())
    }
}
