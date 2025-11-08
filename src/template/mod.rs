use std::{env, fs};

pub mod ec_cli;
pub mod commands;
pub mod runner;

pub use day::*;

mod day;
mod readme_benchmarks;
mod run_multi;
mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string with part suffix. E.g. like `01-1.txt`.
#[must_use]
pub fn read_file(folder: &str, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Creates the constant `DAY` and sets up the input and runner for each part.
///
/// The optional, second parameter (1, 2, or 3) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($day:expr) => {
        $crate::solution!(@impl $day, [part_one, 1] [part_two, 2] [part_three, 3]);
    };
    ($day:expr, 1) => {
        $crate::solution!(@impl $day, [part_one, 1]);
    };
    ($day:expr, 2) => {
        $crate::solution!(@impl $day, [part_two, 2]);
    };
    ($day:expr, 3) => {
        $crate::solution!(@impl $day, [part_three, 3]);
    };

    (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        /// The current day.
        const DAY: $crate::template::Day = $crate::day!($day);

        fn main() {
            use $crate::template::runner::*;
            $(
                let input = $crate::template::read_file("inputs", DAY, $part);
                run_part($func, &input, DAY, $part);
            )*
        }
    };
}
