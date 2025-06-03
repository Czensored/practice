use std::collections::HashSet;

fn apply_choice(board: &HashSet<(usize, usize)>, cell: (usize, usize), n: usize) -> HashSet<(usize, usize)> {
    let mut new_board = board.clone();

    new_board.remove(&cell);
    
    let directions = [
        (0, 1),  (1, 0),
        (1, 1),  (1, -1),
        (-1, 1), (-1, -1),
    ];

    for i in 0..n {
        for &(dy, dx) in &directions {
            let row = cell.0 as isize + dy * i as isize;
            let col = cell.1 as isize + dx * i as isize;

            if row >= 0 && col >= 0 {
                new_board.remove(&(row as usize, col as usize));
            }
        }
    }
    println!("{:?}, cell: {:?}", new_board, cell);

    new_board
}

fn pick_and_step(n: usize, board: HashSet<(usize, usize)>, num_current_queens: usize) -> usize {
    if num_current_queens == n {
        return 1;
    } else if board.is_empty() {
        return 0;
    }
    let mut total = 0;
    for cell in &board {
        // println!("{:?}", cell);
        let new_board = apply_choice(&board, *cell, n);
        total += pick_and_step(n, new_board, num_current_queens + 1);
    }

    total
}

fn num_ways_n_queens(n: usize) -> usize {
    let mut board = HashSet::new();
    for row in 0..n {
        for col in 0..n {
            board.insert((row, col));
        }
    }

    pick_and_step(n, board, 0)
}

fn main() {
    let n = 2;
    let num_ways = num_ways_n_queens(n);
    println!("The number of ways to place {} Queens on an {} by {} chessboard is {}.", n, n, n, num_ways);
}
