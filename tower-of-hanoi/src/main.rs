use std::io::{self, Write};

fn read_two_usize() -> Option<(usize, usize)> {
    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();

    if trimmed.len() != 2 || !trimmed.chars().all(|c| c.is_digit(10)) {
        println!("Please enter exactly two digits (e.g. 01, 12, 20).");
        return None;
    }

    let chars: Vec<char> = trimmed.chars().collect();

    let x = chars[0].to_digit(10)? as usize;
    let y = chars[1].to_digit(10)? as usize;

    if x > 2 || y > 2 {
        println!("Each digit must be 0, 1, or 2.");
        return None;
    }

    Some((x, y))
}

fn print_board(v: &Vec<Vec<usize>>) {
    for (i, tower) in v.iter().enumerate() {
        println!("Pole {}: {:?}", i, tower);
    }
    println!();
}

fn move_disk(
    v: &mut Vec<Vec<usize>>,
    from: usize,
    to: usize,
    moves: &mut usize
) -> bool {
    if v[from].is_empty() {
        println!("Pole {} is empty.", from);
        return false;
    }

    let from_top = *v[from].last().unwrap();

    if let Some(&to_top) = v[to].last() {
        if from_top > to_top {
            println!("Cannot move larger disk onto smaller one.");
            return false;
        }
    }

    let disk = v[from].pop().unwrap();
    v[to].push(disk);
    *moves += 1;

    println!("Moved disk {} from {} to {}", disk, from, to);
    print_board(&v);
    true
}

fn play_manual(v: &mut Vec<Vec<usize>>, n: usize, moves: &mut usize) {
    println!("Manual mode. To input a move, input from and to (e.g. 12 for from=1 to=2)");
    println!("Starting Board");
    print_board(v);

    while v[2].len() != n {
        let Some((x, y)) = read_two_usize() else {
            continue;
        };

        if !move_disk(v, x, y, moves) {
            println!("Try again.");
        }
    }

    println!("Congratulations! Final state:");
    print_board(&v);
    println!("Total moves: {}", moves);
}

fn solve_hanoi(
    n: usize,
    from: usize,
    to: usize,
    aux: usize,
    v: &mut Vec<Vec<usize>>,
    moves: &mut usize,
) {
    if n == 0 {
        return;
    }

    solve_hanoi(n - 1, from, aux, to, v, moves);
    move_disk(v, from, to, moves);
    solve_hanoi(n - 1, aux, to, from, v, moves);
}

fn play_auto(v: &mut Vec<Vec<usize>>, n: usize, moves: &mut usize) {
    println!("Auto-solving...\n");
    println!("Starting Board");
    print_board(&v);
    solve_hanoi(n, 0, 2, 1, v, moves);
    println!("\nDone! Final state: {:?}", v);
    println!("Total moves: {}", moves);
}

fn tower_of_hanoi(n: usize, mode: &str) {
    let mut v = vec![vec![], vec![], vec![]];
    for i in (0..n).rev() {
        v[0].push(i);
    }

    let mut moves = 0;

    match mode {
        "manual" => play_manual(&mut v, n, &mut moves),
        "auto" => play_auto(&mut v, n, &mut moves),
        _ => println!("Unknown mode: choose 'manual' or 'auto'"),
    }
}

fn main() {
    // Change to "manual" or "auto"
    // tower_of_hanoi(3, "manual");
    tower_of_hanoi(6, "auto");
}

