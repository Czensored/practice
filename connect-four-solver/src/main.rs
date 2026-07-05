use connect_four_solver::{GameBoard, Piece};

fn main() {
    let mut gb = GameBoard::new();
    gb.print();
    gb.make_move(Piece::Red, 6);
    gb.make_move(Piece::Yellow, 6);
    gb.print();
    gb.undo_move();
    gb.print();
}
