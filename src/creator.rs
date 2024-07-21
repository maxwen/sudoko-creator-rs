use alloc::vec::Vec;
use core::cmp::PartialEq;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use tracing::info;

use crate::board::{BOARD_BLOCK_SIZE, BOARD_SIZE, SudokuBoard};
use crate::solver::Solver;

#[allow (dead_code)]
pub enum Difficulty {
    EASY = 35,
    MEDIUM = 45,
    HARD = 50,
    EXPERT = 55,
}

#[derive(Debug, Clone, PartialEq)]

pub enum BacktrackingResult {
    /** A result was found, abort. */
    FOUND,
    /** No result found, continue search. */
    CONTINUE,
    /** There's a contradiction in the matrix that can't be solved.
            * */
    CONTRADICTION
}

pub struct Creator {
    pub riddle: SudokuBoard,
}

impl Creator {
    pub fn new() -> Self {
        Creator {
            riddle: SudokuBoard::new(),
        }
    }

    pub fn fill_block(&mut self, row: usize, col: usize) {
        let numbers = self.random_numbers();
        let mut k = 0;
        for i in 0..BOARD_BLOCK_SIZE {
            for j in 0..BOARD_BLOCK_SIZE {
                self.riddle.set(row + i, col + j, numbers[k]);
                k += 1;
            }
        }
    }

    fn random_numbers(&mut self) -> Vec<u8> {
        let mut vec: Vec<u8> = (1..(BOARD_SIZE + 1) as u8).collect();
        vec.shuffle(&mut thread_rng());
        vec
    }

    fn backtrack(&mut self, set_count: u8) -> BacktrackingResult {
        if set_count == 0 {
            return BacktrackingResult::FOUND;
        }

        let least_free_cell = self.riddle.find_least_free_cell();
        if least_free_cell == (-1, -1) {
            return BacktrackingResult::CONTRADICTION;
        }
        let row = least_free_cell.0 as usize;
        let col = least_free_cell.1 as usize;
        let free_nums = self.riddle.free_values(row, col);
        for free_num in free_nums.iter() {
            self.riddle.set(row, col, *free_num);

            let sub_result = self.backtrack(set_count - 1);
            if sub_result == BacktrackingResult::FOUND {
                return sub_result;
            }
        }
        self.riddle.set(row, col, 0);
        BacktrackingResult::CONTINUE
    }
    pub fn create_full(&mut self) -> SudokuBoard {
        loop {
            self.riddle.clear();

            // * 0 0
            // 0 * 0
            // 0 0 *
            //
            // The blocks on the diagonal can be filled independently in random
            // because they can not collide.
            // There can be a contradiction later on anyway.
            for i in 0..BOARD_BLOCK_SIZE {
                self.fill_block(i * BOARD_BLOCK_SIZE, i * BOARD_BLOCK_SIZE)
            }
            let result = self.backtrack(self.riddle.free_count());
            if result == BacktrackingResult::FOUND {
                break
            }
        }
        self.riddle.clone()
    }

    pub fn can_clear(&self, riddle: &mut SudokuBoard, row: usize, col: usize) -> bool
    {
        let free_nums = riddle.free_values(row, col);
        if free_nums.len() == 0 {
            return true;
        }

        let old_val = riddle.get(row, col);
        riddle.set(row, col, 0);

        let mut solver = Solver::new(riddle.clone());
        // simple backtrack solver
        // let solutions = solver.solve();
        // or custom
        let solutions = solver.alt_solve();
        let result = solutions.len() == 1;
        riddle.set(row, col, old_val);
        return result;
    }
    pub fn create_riddle(&mut self, full: SudokuBoard, difficulty: Difficulty) -> SudokuBoard {
        let mut to_remove = difficulty as u8;
        let mut removed = 0;
        let mut riddle = SudokuBoard::new();
        riddle.set_all(full.get_all());

        let mut random_clear_count = 0;
        while random_clear_count < 16 && to_remove > 0{
            let col = thread_rng().gen_range(0..BOARD_SIZE);
            let row = thread_rng().gen_range(0..BOARD_SIZE);

            if riddle.get(row, col) != 0 {
                if self.can_clear(&mut riddle, row, col) {
                    riddle.set(row, col, 0);
                    to_remove -= 1;
                    removed +=1;
                } else {
                    random_clear_count += 1;
                }
            }
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if to_remove > 0
                    && riddle.get(row, col) != 0
                    && self.can_clear(&mut riddle, row, col) {
                    riddle.set(row, col, 0);
                    to_remove -= 1;
                    removed +=1;
                }
            }
        }
        info!("removed = {}", removed);
        riddle
    }
}