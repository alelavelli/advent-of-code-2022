use std::{collections::HashSet, error::Error, fs::File, io::Read, str::FromStr, time::Instant};

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

#[derive(Debug, EnumString)]
enum Direction {
    R,
    L,
    U,
    D,
}

struct Move {
    direction: Direction,
    steps: i32,
}

fn distance(head: &(i32, i32), tail: &(i32, i32)) -> f32 {
    let x_diff = (head.0 - tail.0) as f32;
    let y_diff = (head.1 - tail.1) as f32;
    (x_diff.powi(2) + y_diff.powi(2)).sqrt()
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
}

impl Rope {
    /// Move the tail to match the head
    ///
    /// It takes as input the new position of the head,
    /// the current position of the tail
    fn align(moved_head: (i32, i32), prev_tail: (i32, i32)) -> ((i32, i32), Vec<(i32, i32)>) {
        let mut moved_tail = prev_tail;
        let mut tail_positions: Vec<(i32, i32)> = Vec::new();

        if distance(&moved_head, &prev_tail) > 2.0f32.sqrt() {
            // if the distance between head and tail is greater than sqrt(2) i.e.,
            // neither in the diagonal or adjacent cells we need to move the tail
            //
            // if they are in the same axis then we move the tail in the same direction
            // but one step before
            //
            // otherwise, something more complext needs to be done
            if moved_head.1 == prev_tail.1 {
                if moved_head.0 > prev_tail.0 {
                    moved_tail.0 = moved_head.0 - 1;
                } else {
                    moved_tail.0 = moved_head.0 + 1;
                }

                let (start, end) = if prev_tail.0 > moved_tail.0 {
                    (moved_tail.0, prev_tail.0)
                } else {
                    (prev_tail.0, moved_tail.0)
                };
                tail_positions = (start..=end).map(|x| (x, prev_tail.1)).collect();
            } else if moved_head.0 == prev_tail.0 {
                if moved_head.1 > prev_tail.1 {
                    moved_tail.1 = moved_head.1 - 1;
                } else {
                    moved_tail.1 = moved_head.1 + 1;
                }

                let (start, end) = if prev_tail.1 > moved_tail.1 {
                    (moved_tail.1, prev_tail.1)
                } else {
                    (prev_tail.1, moved_tail.1)
                };
                tail_positions = (start..=end).map(|y| (prev_tail.0, y)).collect();
            } else if (moved_head.0 > prev_tail.0) & (moved_head.1 > prev_tail.1) {
                /* the head is bottom right of tail
                . . T . .
                . . . . H

                first we move one step in the lower diagonal and next we follow head
                */
                moved_tail = (moved_tail.0 + 1, moved_tail.1 + 1);
                tail_positions.push(moved_tail);
                let (next_moved_tail, next_tail_positions) = Rope::align(moved_head, moved_tail);
                moved_tail = next_moved_tail;
                tail_positions.append(&mut next_tail_positions.clone());
            } else if (moved_head.0 < prev_tail.0) & (moved_head.1 < prev_tail.1) {
                /* the head is upper left of tail
                . . H . .
                . . . . T

                first we move one step in the lower diagonal and next we follow head
                */
                moved_tail = (moved_tail.0 - 1, moved_tail.1 - 1);
                tail_positions.push(moved_tail);
                let (next_moved_tail, next_tail_positions) = Rope::align(moved_head, moved_tail);
                moved_tail = next_moved_tail;
                tail_positions.append(&mut next_tail_positions.clone());
            } else if (moved_head.0 > prev_tail.0) & (moved_head.1 < prev_tail.1) {
                /* the head is bottom left of tail
                . . T . .
                H . . . .

                first we move one step in the lower diagonal and next we follow head
                */
                moved_tail = (moved_tail.0 + 1, moved_tail.1 - 1);
                tail_positions.push(moved_tail);
                let (next_moved_tail, next_tail_positions) = Rope::align(moved_head, moved_tail);
                moved_tail = next_moved_tail;
                tail_positions.append(&mut next_tail_positions.clone());
            } else {
                /* the head is upper right of tail
                . . H . .
                T . . . .

                first we move one step in the lower diagonal and next we follow head
                */
                moved_tail = (moved_tail.0 - 1, moved_tail.1 + 1);
                tail_positions.push(moved_tail);
                let (next_moved_tail, next_tail_positions) = Rope::align(moved_head, moved_tail);
                moved_tail = next_moved_tail;
                tail_positions.append(&mut next_tail_positions.clone());
            }
        }
        (moved_tail, tail_positions)
    }

    fn apply_move(&mut self, move_to_apply: &Move) -> HashSet<(i32, i32)> {
        let prev_head = self.head;
        let prev_tail = self.tail;

        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

        let (x_step, y_step) = match move_to_apply.direction {
            Direction::U => (-move_to_apply.steps, 0),
            Direction::L => (0, -move_to_apply.steps),
            Direction::R => (0, move_to_apply.steps),
            Direction::D => (move_to_apply.steps, 0),
        };

        let moved_head = (prev_head.0 + x_step, prev_head.1 + y_step);
        self.head = moved_head;
        let (new_tail, new_tail_positions) = Rope::align(moved_head, prev_tail);
        self.tail = new_tail;
        for tail_pos in new_tail_positions {
            if !tail_positions.contains(&tail_pos) {
                tail_positions.insert(tail_pos);
            }
        }
        tail_positions
    }
}

struct LongRope {
    head: (i32, i32),
    tails: Vec<(i32, i32)>,
}

impl LongRope {
    fn apply_move(&mut self, move_to_apply: &Move) -> HashSet<(i32, i32)> {
        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
        for _ in 0..move_to_apply.steps {
            // we need to do this because the tail can move in strange ways at each step
            // if we only look at the last position of a knot we can miss the actual path
            let prev_head = self.head;
            let (x_step, y_step) = match move_to_apply.direction {
                Direction::U => (-1, 0),
                Direction::L => (0, -1),
                Direction::R => (0, 1),
                Direction::D => (1, 0),
            };

            let moved_head = (prev_head.0 + x_step, prev_head.1 + y_step);
            self.head = moved_head;

            let mut new_tails = Vec::new();
            let mut last_tail_positions = Vec::new();

            let (new_tail, _) = Rope::align(moved_head, self.tails[0]);
            new_tails.push(new_tail);
            let mut prev_tail = new_tail;
            for current_tail in self.tails.iter().skip(1) {
                let (new_tail, new_tail_positions) = Rope::align(prev_tail, *current_tail);
                new_tails.push(new_tail);
                prev_tail = new_tail;
                last_tail_positions = new_tail_positions;
            }

            for tail_pos in last_tail_positions {
                if !tail_positions.contains(&tail_pos) {
                    tail_positions.insert(tail_pos);
                }
            }

            self.tails = new_tails;
        }
        tail_positions
    }
}

fn parse_input(puzzle_input: String) -> Vec<Move> {
    let mut moves = Vec::new();
    for line in puzzle_input.lines() {
        moves.push(Move {
            direction: Direction::from_str(line.split_whitespace().next().unwrap()).unwrap(),
            steps: line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        });
    }
    moves
}

fn print_positions(tail_positions: &HashSet<(i32, i32)>) {
    let min_x = tail_positions.iter().map(|x| x.0).min().unwrap();
    let max_x = tail_positions.iter().map(|x| x.0).max().unwrap();
    let min_y = tail_positions.iter().map(|x| x.1).min().unwrap();
    let max_y = tail_positions.iter().map(|x| x.1).max().unwrap();
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if (i == 0) & (j == 0) {
                print!("s");
            } else if tail_positions.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let moves = parse_input(puzzle_input);
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));
    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
    };
    for move_to_apply in moves {
        let new_tail_positions = rope.apply_move(&move_to_apply);
        tail_positions.extend(&new_tail_positions);
    }
    println!("{:?}", tail_positions);
    print_positions(&tail_positions);
    Ok(tail_positions.len().to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let moves = parse_input(puzzle_input);
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));
    let mut rope = LongRope {
        head: (0, 0),
        tails: vec![(0, 0); 9],
    };

    for move_to_apply in moves {
        let new_tail_positions = rope.apply_move(&move_to_apply);
        tail_positions.extend(&new_tail_positions);
    }
    println!("{:?}", tail_positions);
    print_positions(&tail_positions);
    Ok(tail_positions.len().to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_09_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("13".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_09_example_2.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("36".to_string(), result);

        Ok(())
    }
}
