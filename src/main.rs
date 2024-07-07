extern crate alloc;
extern crate core;

use sudoku::Sudoku;
use crate::creator::{Creator, Difficulty};

mod board;
mod creator;
mod solver;

fn main() {
    let mut c = Creator::new();
    let full = c.create_full();
    println!("{}", full);

    if full.is_valid() {
        let mut riddle = c.create_riddle(full.clone(), Difficulty::MEDIUM);
        println!("{}", riddle);
        println!("{}", riddle.to_line_format().as_str());
        let sudoku = Sudoku::from_str_line(riddle.to_line_format().as_str()).unwrap();
        // Solve, print or convert the sudoku to another format
        if let Some(solution) = sudoku.some_solution() {
            // print the solution in line format
            println!("{}", solution);
            let _ = riddle.from_line_format(solution.to_str_line().as_bytes());
            println!("{}", riddle);

            println!("equals = {}", full == riddle);
        }
    }
}
