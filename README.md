# Advent of code 2022
My implementations of Advent of Code year 2022.

## Structure

The repo is a binary crate with a CLI which executes problems.
Arguments are:
- day: daily problem to solve
- part: if part `one` or part `two`

For instance:
```bash
cargo run -- --day 1 --part one
```

Each day is a module with two private functions to implement: `solve_pt1` and `solve_pt2`. 
They take puzzle input as a `String` and return `Result<String, Box<dyn Error>>` with the result of the problem.
```rs
fn solve_pt1(_puzzle_input: String) -> Result<String, Box<dyn Error>> {
    todo!()
}

fn solve_pt2(_puzzle_input: String) -> Result<String, Box<dyn Error>> {
    todo!()
}
```
Test module allows to test the functions with the example file provided by the problem instructions.