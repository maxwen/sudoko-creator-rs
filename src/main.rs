#![no_std]

extern crate alloc;
extern crate core;

use sudoku::Sudoku;
use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::creator::{Creator, Difficulty};

mod board;
mod creator;
mod solver;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .init();

    let mut c = Creator::new();
    let full = c.create_full();
    info!("{}", full);

    if full.is_valid() {
        let mut riddle = c.create_riddle(full.clone(), Difficulty::MEDIUM);
        info!("{}", riddle);
        info!("{}", riddle.to_line_format().as_str());
        let sudoku = Sudoku::from_str_line(riddle.to_line_format().as_str()).unwrap();
        // Solve, print or convert the sudoku to another format
        if let Some(solution) = sudoku.some_solution() {
            // print the solution in line format
            info!("{}", solution);
            let _ = riddle.from_line_format(solution.to_str_line().as_bytes());
            info!("{}", riddle);

            info!("equals = {}", full == riddle);
        }
    }
}
