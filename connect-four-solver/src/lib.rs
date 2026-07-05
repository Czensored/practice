#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Red,
    Yellow,
    Null,
}

pub struct GameBoard {
    pub gb: [[Piece; 7]; 6],
    pub height_arr: [usize; 7],
    history: Vec<([[Piece; 7]; 6], [usize; 7])>,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            gb: [[Piece::Null; 7]; 6],
            height_arr: [0; 7],
            history: Vec::new(),
        }
    }

    pub fn print(&self) {
        for row in self.gb {
            for col in row {
                let pce = match col {
                    Piece::Red => 'R',
                    Piece::Yellow => 'Y',
                    Piece::Null => 'O',
                };
                print!("{} ", pce);
            }
            println!();
        }
        println!();
    }

    pub fn make_move(&mut self, player: Piece, col: usize) {
        if self.height_arr[col] == self.gb.len() {
            return;
        }
        self.history.push((self.gb, self.height_arr));
        let row = self.gb.len() - 1 - self.height_arr[col];
        self.gb[row][col] = player;
        self.height_arr[col] += 1;
    }

    pub fn undo_move(&mut self) {
        if let Some((gb, height_arr)) = self.history.pop() {
            self.gb = gb;
            self.height_arr = height_arr;
        }
    }

    pub fn check_win(&self) -> Option<Piece> {
        let row_len = self.gb.len();
        let col_len = self.gb[0].len();
        let directions: [(isize, isize); 4] = [
            (0, 1),  // horizontal
            (1, 0),  // vertical
            (1, 1),  // diagonal down-right
            (-1, 1), // diagonal up-right
        ];

        for col in 0..col_len {
            for row in row_len - self.height_arr[col]..row_len {
                let piece = self.gb[row][col];

                if piece == Piece::Null {
                    continue;
                }

                for (row_delta, col_delta) in directions {
                    let mut found_win = true;

                    for distance in 1..4 {
                        let distance = distance as isize;
                        let next_row = row as isize + row_delta * distance;
                        let next_col = col as isize + col_delta * distance;

                        if next_row < 0
                            || next_row >= row_len as isize
                            || next_col < 0
                            || next_col >= col_len as isize
                            || self.gb[next_row as usize][next_col as usize] != piece
                        {
                            found_win = false;
                            break;
                        }
                    }

                    if found_win {
                        return Some(piece);
                    }
                }
            }
        }

        None
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
