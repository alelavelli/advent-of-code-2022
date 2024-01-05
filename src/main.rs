use advent_of_code_template::*;
use clap::Parser;
use log::error;

fn main() {
    // Parse arguments
    let args = CliArgs::parse();

    // Set logger
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let result = match args.day {
        0 => day_0::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_0.txt")),
            args.part,
        ),
        1 => day_01::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_01.txt")),
            args.part,
        ),
        2 => day_02::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_02.txt")),
            args.part,
        ),
        3 => day_03::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_03.txt")),
            args.part,
        ),
        4 => day_04::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_04.txt")),
            args.part,
        ),
        5 => day_05::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_05.txt")),
            args.part,
        ),
        6 => day_06::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_06.txt")),
            args.part,
        ),
        7 => day_07::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_07.txt")),
            args.part,
        ),
        8 => day_08::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_08.txt")),
            args.part,
        ),
        9 => day_09::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_09.txt")),
            args.part,
        ),
        10 => day_10::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_10.txt")),
            args.part,
        ),
        11 => day_11::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_11.txt")),
            args.part,
        ),
        12 => day_12::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_12.txt")),
            args.part,
        ),
        13 => day_13::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_13.txt")),
            args.part,
        ),
        14 => day_14::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_14.txt")),
            args.part,
        ),
        15 => day_15::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_15.txt")),
            args.part,
        ),
        16 => day_16::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_16.txt")),
            args.part,
        ),
        17 => day_17::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_17.txt")),
            args.part,
        ),
        18 => day_18::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_18.txt")),
            args.part,
        ),
        19 => day_19::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_19.txt")),
            args.part,
        ),
        20 => day_20::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_20.txt")),
            args.part,
        ),
        21 => day_21::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_21.txt")),
            args.part,
        ),
        22 => day_22::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_22.txt")),
            args.part,
        ),
        23 => day_23::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_23.txt")),
            args.part,
        ),
        24 => day_24::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_24.txt")),
            args.part,
        ),
        25 => day_25::solve(
            &args
                .puzzle_input
                .unwrap_or(String::from("inputs/day_25.txt")),
            args.part,
        ),
        _ => {
            error!("Ops, you submitted a wrong day! Retry a number between 0 and 25 ");
            Ok(())
        }
    };
    if let Err(error) = result {
        error!("Ops, something goes wrong. Error is {error}");
    }
}
