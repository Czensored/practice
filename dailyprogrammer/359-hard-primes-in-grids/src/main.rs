// Ref: https://www.reddit.com/r/dailyprogrammer/comments/8gzaz5/20180504_challenge_359_hard_primes_in_grids/
// Ref: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

use primality_utils::miller_rabin;
use num_bigint::BigUint;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn parse_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn collect_primes_from_grid(grid: &[Vec<char>]) -> HashSet<BigUint> {
    let mut primes = HashSet::new();
    let n = grid.len();

    let directions = [
        (0, 1),   // →
        (0, -1),  // ←
        (1, 0),   // ↓
        (-1, 0),  // ↑
        (1, 1),   // ↘
        (-1, -1), // ↖
        (1, -1),  // ↙
        (-1, 1),  // ↗
    ];

    for row in 0..n {
        for col in 0..n {
            for &(dr, dc) in &directions {
                let mut digits = String::new();
                let (mut r, mut c) = (row as isize, col as isize);

                while r >= 0 && r < n as isize && c >= 0 && c < n as isize {
                    digits.push(grid[r as usize][c as usize]);
                    let num = BigUint::parse_bytes(digits.as_bytes(), 10).unwrap();
                    if miller_rabin(&num, 10) {
                        primes.insert(num.clone());
                    }
                    r += dr;
                    c += dc;
                }
            }
        }
    }

    primes
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let grid = parse_grid(lines);

    let start = Instant::now();
    let primes = collect_primes_from_grid(&grid);
    let duration = start.elapsed();

    println!("Found {} distinct primes.", primes.len());
    println!("Search took {:.3?} seconds.", duration);

    Ok(())
}
