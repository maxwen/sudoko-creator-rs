use crate::board::SudokuBoard;

pub struct Solver {
    riddle: SudokuBoard,
    solutions: Vec<SudokuBoard>,
}

impl Solver {
    pub fn new(riddle: SudokuBoard) -> Self {
        Solver {
            riddle,
            solutions: vec![],
        }
    }

    pub fn solve(&mut self) -> Vec<SudokuBoard> {
        self.solutions.clear();

        self.backtrack(self.riddle.free_count());
        self.solutions.clone()
    }

    fn backtrack(&mut self, free_count: u8) -> u8 {
        if self.solutions.len() > 0 {
            return 1;
        }

        if free_count == 0 {
            self.solutions.push(self.riddle.clone());
            return 1;
        }

        let least_free_cell = self.riddle.find_least_free_cell();
        if (least_free_cell == (-1, -1)) {
            return 0;
        }

        let mut result = 0;
        let row = least_free_cell.0 as usize;
        let col = least_free_cell.1 as usize;
        let free_nums = self.riddle.free_values(row, col);
        for free_num in free_nums.iter() {
            self.riddle.set(row, col, *free_num);

            let result_count = self.backtrack(free_count - 1);
            result += result_count;
        }
        self.riddle.set(row, col, 0);
        result
    }
}