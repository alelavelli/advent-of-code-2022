use std::{collections::HashSet, error::Error, fs::File, io::Read, time::Instant};

use log::info;
use ndarray::{s, Array2, ArrayView2};

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

fn parse_input(puzzle_input: String) -> Array2<i32> {
    let mut matrix = Array2::zeros((
        puzzle_input.lines().collect::<Vec<&str>>().len(),
        puzzle_input.lines().next().unwrap().len(),
    ));
    for (i, line) in puzzle_input.lines().enumerate() {
        for (j, el) in line.chars().enumerate() {
            matrix[(i, j)] = el.to_digit(10).unwrap() as i32;
        }
    }
    matrix
}

fn find_visible_trees(matrix: ArrayView2<i32>) -> HashSet<(usize, usize)> {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..matrix.shape()[0] {
        visible_trees.insert((r, 0));
        visible_trees.insert((r, matrix.shape()[1] - 1));
    }
    for c in 0..matrix.shape()[1] {
        visible_trees.insert((0, c));
        visible_trees.insert((matrix.shape()[0] - 1, c));
    }

    // LEFT
    let mut max_trees = matrix.slice(s![.., 0]).to_owned();
    for c in 0..matrix.shape()[1] {
        for r in 0..matrix.shape()[0] {
            if (!visible_trees.contains(&(r, c))) & (matrix[(r, c)] > max_trees[r]) {
                visible_trees.insert((r, c));
            }
            if max_trees[r] < matrix[(r, c)] {
                max_trees[r] = matrix[(r, c)];
            }
        }
    }

    // RIGHT
    let mut max_trees = matrix.slice(s![.., -1]).to_owned();
    for c in (0..matrix.shape()[1]).rev() {
        for r in 0..matrix.shape()[0] {
            if (!visible_trees.contains(&(r, c))) & (matrix[(r, c)] > max_trees[r]) {
                visible_trees.insert((r, c));
            }
            if max_trees[r] < matrix[(r, c)] {
                max_trees[r] = matrix[(r, c)];
            }
        }
    }

    // UP
    let mut max_trees = matrix.slice(s![0, ..]).to_owned();
    for r in 0..matrix.shape()[0] {
        for c in 0..matrix.shape()[1] {
            if (!visible_trees.contains(&(r, c))) & (matrix[(r, c)] > max_trees[c]) {
                visible_trees.insert((r, c));
            }
            if max_trees[c] < matrix[(r, c)] {
                max_trees[c] = matrix[(r, c)];
            }
        }
    }

    // DOWN
    let mut max_trees = matrix.slice(s![-1, ..]).to_owned();
    for r in (0..matrix.shape()[0]).rev() {
        for c in 0..matrix.shape()[1] {
            if (!visible_trees.contains(&(r, c))) & (matrix[(r, c)] > max_trees[c]) {
                visible_trees.insert((r, c));
            }
            if max_trees[c] < matrix[(r, c)] {
                max_trees[c] = matrix[(r, c)];
            }
        }
    }

    visible_trees
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let matrix = parse_input(puzzle_input);
    let visible_trees = find_visible_trees(matrix.view());

    Ok(visible_trees.len().to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let matrix = parse_input(puzzle_input);
    let visible_trees = find_visible_trees(matrix.view());

    let mut highest_scene = 0;

    for &(tree_r, tree_c) in visible_trees.iter().filter(|(r, c)| {
        (*r > 0) & (*r < matrix.shape()[0] - 1) & (*c > 0) & (*c < matrix.shape()[1] - 1)
    }) {
        // UP
        let mut upper_view = 0;
        for r in (0..tree_r).rev() {
            upper_view += 1;
            if matrix[(r, tree_c)] >= matrix[(tree_r, tree_c)] {
                break;
            }
        }

        // DOWN
        let mut lower_view = 0;
        for r in (tree_r + 1)..matrix.shape()[0] {
            lower_view += 1;
            if matrix[(r, tree_c)] >= matrix[(tree_r, tree_c)] {
                break;
            }
        }

        // RIGHT
        let mut right_view = 0;
        for c in (tree_c + 1)..matrix.shape()[1] {
            right_view += 1;
            if matrix[(tree_r, c)] >= matrix[(tree_r, tree_c)] {
                break;
            }
        }

        // LEFT
        let mut left_view = 0;
        for c in (0..tree_c).rev() {
            left_view += 1;
            if matrix[(tree_r, c)] >= matrix[(tree_r, tree_c)] {
                break;
            }
        }

        let scene = upper_view * lower_view * left_view * right_view;
        if scene > highest_scene {
            highest_scene = scene;
        }
    }

    Ok(highest_scene.to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_08_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("21".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_08_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("8".to_string(), result);

        Ok(())
    }
}
