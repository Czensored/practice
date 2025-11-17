use std::fmt::Write;

pub struct SudokuBoard {
    board: [[usize; 9]; 9],
    past_boards: Vec<[[usize; 9]; 9]>
}

impl SudokuBoard {
    #[allow(dead_code)]
    fn new() -> SudokuBoard {
        SudokuBoard {
            board: [[0; 9]; 9],
            past_boards: Vec::new(),
        }
    }

    // This is the main constructor function
    pub fn from_board(in_board: [[usize; 9]; 9]) -> SudokuBoard {
        SudokuBoard {
            board: in_board,
            past_boards: Vec::new(),
        }
    }

    // pub fn print(&self) {
    //     for (i, arr) in self.board.iter().enumerate() {
    //         if i != 0 && i % 3 == 0 {
    //             println!("------+-------+------");
    //         }
    //         for (j, val) in arr.iter().enumerate() {
    //             if j != 0 && j % 3 == 0 {
    //                 print!("| ");
    //             }
    //             if *val != 0 {
    //                 print!("{val} ");
    //             } else {
    //                 print!(". ");
    //             }
    //         }
    //         println!();
    //     }
    // }

    pub fn print(&self) {
        let mut out = String::new();

        for (i, arr) in self.board.iter().enumerate() {
            if i != 0 && i % 3 == 0 {
                out.push_str("------+-------+------\n");
            }
            for (j, val) in arr.iter().enumerate() {
                if j != 0 && j % 3 == 0 {
                    out.push_str("| ");
                }
                if *val != 0 {
                    let _ = write!(out, "{} ", val);
                } else {
                    out.push_str(". ");
                }
            }

            out.push('\n');
        }

        print!("{}", out);
    }

    pub fn go(&mut self, row: usize, col: usize, num: usize) -> bool {
        if !self.valid_move(row, col, num) {
            return false;
        }

        self.past_boards.push(self.board);
        self.board[row][col] = num;
        true
    }

    pub fn undo(&mut self) -> bool {
        if let Some(last_board) = self.past_boards.pop() {
            self.board = last_board;
            return true;
        }
        false
    }

    pub fn valid_move(&self, row: usize, col: usize, num: usize) -> bool {
        // Check indices in bounds
        if row > 8 || col > 8 {
            return false;
        }

        // Check legal number
        if num < 1 || num > 9 {
            return false;
        }

        // Not already used
        if self.board[row][col] != 0 {
            return false;
        }

        for i in 0..9 {
            // Used in row
            if self.board[i][col] == num {
                return false;
            }
            // Used in col
            if self.board[row][i] == num {
                return false;
            }
        }

        // Used in box
        let box_start_row = row - (row % 3);
        let box_start_col = col - (col % 3);
        for i in 0..3 {
            for j in 0..3 {
                if self.board[box_start_row + i][box_start_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_val(&self, row: usize, col: usize) -> usize {
        assert!(row < 9, "Row index should be less than 9");
        assert!(col < 9, "Column index should be less than 9");
        self.board[row][col]
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.get_val(row, col) == 0
    }

    // Provides a heuristic way of judging how difficult a puzzle is
    #[allow(dead_code)]
    pub fn print_sparseness(&self) {
        let empty_cells = self.board.iter().flatten().filter(|&&cell| cell == 0).count();
        let total_cells: f64 = 81.0;
        let sparseness_percentage = (empty_cells as f64 / total_cells) * 100.0;
        println!("Sparseness of the board: {:.2}%", sparseness_percentage);
    }
}
