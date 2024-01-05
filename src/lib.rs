pub mod day_0;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use strum_macros::{Display, EnumString};

/// Arguments to pass to cli application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Day to solve
    #[arg(short, long)]
    pub day: u8,
    /// Part to solve
    #[arg(short = 'p', long)]
    pub part: ProblemPart,
    /// The file of the input file
    #[arg(short = 'i', long)]
    pub puzzle_input: Option<String>,
    /// Set verbosity level of the application
    ///
    /// -q silences output
    /// -v show warnings
    /// -vv show info
    /// -vvv show debug
    /// -vvvv show trace
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(EnumString, Display, Clone, Debug)]
pub enum ProblemPart {
    #[strum(ascii_case_insensitive)]
    One,
    #[strum(ascii_case_insensitive)]
    Two,
}
