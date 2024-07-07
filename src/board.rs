use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
// use core::hash::Hash;

pub const BOARD_SIZE: usize = 9;
pub const BOARD_BLOCK_SIZE: usize = 3;

pub type SudokuMatrix = [[u8; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone, PartialEq)]
pub struct SudokuBoard {
    ans: SudokuMatrix,
}

#[allow(dead_code)]
impl SudokuBoard {
    pub fn new() -> Self {
        SudokuBoard {
            ans: [[0u8; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    pub fn clear(&mut self) {
        self.ans = [[0u8; BOARD_SIZE]; BOARD_SIZE]
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.ans[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.ans[row][col] = val
    }

    pub fn can_set(&self, row: usize, col: usize, val: u8) -> bool {
        self.free_values(row, col).contains(&val)
    }

    pub fn free_values(&self, row: usize, col: usize) -> Vec<u8> {
        let row_free = self.row_free(row);
        let col_free = self.col_free(col);
        let row_col_free = row_free.iter().filter(|&value| col_free.contains(value)).map(|&value| value).collect::<Vec<u8>>();
        let block_free = self.block_free(row, col);
        let row_col_block_free = row_col_free.iter().filter(|&value| block_free.contains(value)).map(|&value| value).collect::<Vec<u8>>();
        row_col_block_free
    }

    fn has_unique_elements(&self, list: [u8; BOARD_SIZE]) -> bool {
        for i in 1..list.len() {
            if list[i..].contains(&list[i - 1]) {
                return false;
            }
        }
        true
    }
    fn index_of_value(&self, list: &Vec<u8>, val: u8) -> Option<usize> {
        list.iter().position(|&value| value == val)
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..BOARD_SIZE {
            let row = self.row(i);
            if !self.has_unique_elements(row) || row.contains(&0) {
                return false;
            }
        }
        for i in 0..BOARD_SIZE {
            let col = self.col(i);
            if !self.has_unique_elements(col) || col.contains(&0) {
                return false;
            }
        }
        for i in (0..BOARD_SIZE).step_by(BOARD_BLOCK_SIZE) {
            for j in (0..BOARD_SIZE).step_by(BOARD_BLOCK_SIZE) {
                let block = self.block(i, j);
                if !self.has_unique_elements(block) || block.contains(&0) {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_all(&mut self, values: [[u8; BOARD_SIZE]; BOARD_SIZE]) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                self.ans[row][col] = values[row][col]
            }
        }
    }

    pub fn get_all(&self) -> [[u8; BOARD_SIZE]; BOARD_SIZE] {
        let mut ans = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                ans[row][col] = self.ans[row][col]
            }
        }
        ans
    }

    pub fn row(&self, row: usize) -> [u8; BOARD_SIZE] {
        let mut ans = [0u8; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            ans[i] = self.ans[row][i]
        }
        ans
    }

    pub fn col(&self, col: usize) -> [u8; BOARD_SIZE] {
        let mut ans = [0u8; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            ans[i] = self.ans[i][col]
        }
        ans
    }

    fn round_to_block(&self, index: usize) -> usize {
        index - index % BOARD_BLOCK_SIZE
    }

    pub fn block(&self, row: usize, col: usize) -> [u8; BOARD_SIZE] {
        let row = self.round_to_block(row);
        let col = self.round_to_block(col);
        let mut ans = [0u8; BOARD_SIZE];
        let mut k = 0;
        for i in 0..BOARD_BLOCK_SIZE {
            for j in 0..BOARD_BLOCK_SIZE {
                ans[k] = self.ans[row + i][col + j];
                k += 1;
            }
        }
        ans
    }

    fn get_unset(&self, ans: [u8; BOARD_SIZE]) -> Vec<u8> {
        let mut unset: Vec<u8> = (1..(BOARD_SIZE + 1) as u8).collect();
        for i in 0..BOARD_SIZE {
            let val = ans[i];
            if val != 0 {
                if let Some(index) = self.index_of_value(&unset, val) {
                    unset.remove(index);
                }
            }
        }
        unset
    }

    pub fn row_free(&self, row: usize) -> Vec<u8> {
        self.get_unset(self.row(row))
    }

    pub fn col_free(&self, col: usize) -> Vec<u8> {
        self.get_unset(self.col(col))
    }

    pub fn block_free(&self, row: usize, col: usize) -> Vec<u8> {
        self.get_unset(self.block(row, col))
    }

    pub fn set_count(&self) -> u8 {
        let mut k = 0;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.ans[row][col] != 0 {
                    k += 1;
                }
            }
        }
        k
    }

    pub fn free_count(&self) -> u8 {
        (BOARD_SIZE * BOARD_SIZE) as u8 - self.set_count()
    }

    /* -1,-1 non free
     */
    pub fn find_least_free_cell(&self) -> (i8, i8) {
        let mut row_col_result = (-1i8, -1i8);
        let mut min_free_size = BOARD_SIZE;

        for row in 0..BOARD_SIZE {
            if self.row_free(row).len() == 0 {
                continue;
            }
            for col in 0..BOARD_SIZE {
                if self.get(row, col) != 0 {
                    continue;
                }
                let free = self.free_values(row, col);
                if free.len() == 0 {
                    return (-1, -1);
                }
                if free.len() == 1 {
                    row_col_result = (row as i8, col as i8);
                    return row_col_result;
                }
                if free.len() < min_free_size {
                    row_col_result = (row as i8, col as i8);
                    min_free_size = free.len();
                }
            }
        }

        row_col_result
    }

    pub fn to_line_format(&self) -> String {
        let mut line = String::new();
        for i in 0..BOARD_SIZE {
            let row = self.row(i);
            for val in row {
                line.push(to_line_char(val))
            }
        }
        line
    }

    pub fn from_line_format(&mut self, chars: &[u8]) -> Result<(), ()> {
        self.clear();

        if chars.len() < BOARD_SIZE * BOARD_SIZE {
            return Err(());
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let cell = chars[row * BOARD_SIZE + col];
                let mut val = cell.wrapping_sub(b'0');
                if val == b'_' - b'0' {
                    val = 0;
                }
                if val == b'.'.wrapping_sub(b'0') {
                    val = 0;
                }
                self.set(row, col, val);
            }
        }
        Ok(())
    }
}

fn line(start: char, thick_sep: char, thin_sep: char,
        segment: impl Fn(usize) -> char, pad: char, end: char, newline: bool) -> String {
    let size = BOARD_SIZE;
    let mut result = String::new();

    for x in 0..size {
        if x == 0 {
            result.push(start);
        } else if x % BOARD_BLOCK_SIZE == 0 {
            result.push(thick_sep);
        } else {
            result.push(thin_sep);
        }

        result.push(pad);
        result.push(segment(x));
        result.push(pad);
    }

    result.push(end);

    if newline {
        result.push('\n');
    }

    result
}

fn top_row() -> String {
    line('╔', '╦', '╤', |_| '═', '═', '╗', true)
}

fn thin_separator_line() -> String {
    line('╟', '╫', '┼', |_| '─', '─', '╢', true)
}

fn thick_separator_line() -> String {
    line('╠', '╬', '╪', |_| '═', '═', '╣', true)
}

fn bottom_row() -> String {
    line('╚', '╩', '╧', |_| '═', '═', '╝', false)
}

fn content_row(grid: &SudokuBoard, row: usize) -> String {
    line('║', '║', '│', |col| to_char(grid.get(row, col)), ' ',
         '║', true)
}

fn to_char(val: u8) -> char {
    if val != 0 {
        (b'0' + val) as char
    } else {
        ' '
    }
}

fn to_line_char(val: u8) -> char {
    if val != 0 {
        (b'0' + val) as char
    } else {
        '.'
    }
}


impl Display for SudokuBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let top_row = top_row();
        let thin_separator_line = thin_separator_line();
        let thick_separator_line = thick_separator_line();
        let bottom_row = bottom_row();

        for row in 0..BOARD_SIZE {
            if row == 0 {
                f.write_str(top_row.as_str())?;
            } else if row % BOARD_BLOCK_SIZE == 0 {
                f.write_str(thick_separator_line.as_str())?;
            } else {
                f.write_str(thin_separator_line.as_str())?;
            }

            f.write_str(content_row(self, row).as_str())?;
        }

        f.write_str(bottom_row.as_str())?;
        Ok(())
    }
}