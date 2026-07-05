use super::{GameBoard, Piece};

#[test]
fn check_win_finds_horizontal_win() {
    let mut gb = GameBoard::new();

    for col in 0..4 {
        gb.make_move(Piece::Red, col);
    }

    assert!(matches!(gb.check_win(), Some(Piece::Red)));
}

#[test]
fn check_win_finds_vertical_win() {
    let mut gb = GameBoard::new();

    for _ in 0..4 {
        gb.make_move(Piece::Yellow, 0);
    }

    assert!(matches!(gb.check_win(), Some(Piece::Yellow)));
}

#[test]
fn check_win_finds_diagonal_win() {
    let mut gb = GameBoard::new();

    gb.make_move(Piece::Yellow, 3);
    gb.make_move(Piece::Red, 2);
    gb.make_move(Piece::Yellow, 2);
    gb.make_move(Piece::Red, 1);
    gb.make_move(Piece::Red, 1);
    gb.make_move(Piece::Yellow, 1);
    gb.make_move(Piece::Red, 0);
    gb.make_move(Piece::Red, 0);
    gb.make_move(Piece::Red, 0);
    gb.make_move(Piece::Yellow, 0);

    assert!(matches!(gb.check_win(), Some(Piece::Yellow)));
}

#[test]
fn check_win_checks_top_row_of_full_columns() {
    let mut gb = GameBoard::new();

    for col in 0..4 {
        gb.gb[0][col] = Piece::Red;
        gb.height_arr[col] = gb.gb.len();
    }

    assert!(matches!(gb.check_win(), Some(Piece::Red)));
}

#[test]
fn check_win_returns_none_without_winner() {
    let gb = GameBoard::new();

    assert!(matches!(gb.check_win(), None));
}
