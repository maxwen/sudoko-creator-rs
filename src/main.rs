use crate::creator::{Creator, Difficulty};

mod board;
mod creator;
mod solver;

fn main() {
    let full = Creator::create_full();
    println!("{}", full);
    if full.is_valid() {
        let riddle = Creator::create_riddle(full, Difficulty::MEDIUM);
        println!("{}", riddle);
    }
}
