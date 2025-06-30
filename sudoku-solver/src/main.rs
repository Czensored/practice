mod sudoku_board;
use sudoku_board::SudokuBoard;

fn find_unassigned(s_board: &SudokuBoard, row: &mut usize, col: &mut usize) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if s_board.is_empty(i, j) {
                *row = i;
                *col = j;
                return true;
            }
        }
    }
    false
}

fn solve_sudoku(s_board: &mut SudokuBoard) -> bool {
    let mut row = 0;
    let mut col = 0;

    if !find_unassigned(s_board, &mut row, &mut col) {
        return true;
    }

    for i in 1..=9 {
        if s_board.go(row, col, i) {
            if solve_sudoku(s_board) {
                return true;
            }
            s_board.undo();
        }
    }

    false
}

fn solve_with_print(s_board: &mut SudokuBoard) {
    s_board.print();
    println!();
    println!();
    solve_sudoku(s_board);
    s_board.print();
    println!();
}

fn main() {
    // let mut solve = SudokuBoard::from_board(
    //     [
    //         [0, 0, 0, 7, 0, 0, 8, 0, 6],
    //         [0, 0, 5, 8, 6, 1, 0, 0, 3],
    //         [0, 7, 0, 4, 0, 0, 0, 0, 0],
    //         [0, 0, 0, 1, 7, 0, 0, 2, 8],
    //         [8, 0, 0, 0, 0, 4, 6, 0, 1],
    //         [7, 0, 0, 0, 0, 6, 4, 0, 0],
    //         [4, 5, 0, 0, 0, 7, 0, 0, 9],
    //         [3, 8, 7, 6, 0, 2, 5, 0, 0],
    //         [0, 9, 0, 0, 4, 0, 3, 0, 7]
    //     ]
    // );
    let mut solve = SudokuBoard::from_board(
        [
            [8, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 3, 6, 0, 0, 0, 0, 0],
            [0, 7, 0, 0, 9, 0, 2, 0, 0],
            [0, 5, 0, 0, 0, 7, 0, 0, 0],
            [0, 0, 0, 0, 4, 5, 7, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 3, 0],
            [0, 0, 1, 0, 0, 0, 0, 6, 8],
            [0, 0, 8, 5, 0, 0, 0, 1, 0],
            [0, 9, 0, 0, 0, 0, 4, 0, 0]
        ]
    );

    // let mut solve = SudokuBoard::from_board(
    //     [
    //         [0, 0, 0, 0, 0, 0, 0, 0, 8],
    //         [0, 7, 0, 0, 0, 5, 0, 0, 0],
    //         [2, 0, 0, 3, 0, 0, 0, 4, 0],
    //         [0, 2, 3, 0, 0, 0, 0, 0, 0],
    //         [9, 0, 0, 4, 0, 0, 6, 0, 7],
    //         [0, 8, 0, 0, 0, 0, 5, 2, 0],
    //         [0, 0, 0, 8, 1, 3, 0, 0, 0],
    //         [4, 0, 9, 2, 0, 0, 0, 8, 0],
    //         [0, 0, 0, 0, 0, 0, 2, 0, 0]
    //     ]
    // );

    solve_with_print(&mut solve);
}
