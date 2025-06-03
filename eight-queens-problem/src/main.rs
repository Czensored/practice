mod implementations {
    pub mod list;
    pub mod bitmask;
    pub mod optimized_bitmask;
}

use std::time::Instant;

fn benchmark<F: Fn(usize) -> usize>(name: &str, f: F, n: usize) {
    let start = Instant::now();
    let result = f(n);
    let duration = start.elapsed();
    println!("{}: {} solutions in {:?}", name, result, duration);
}

fn main() {
    let n = 14;

    benchmark("List", implementations::list::num_ways_n_queens, n);
    benchmark("Bitmask", implementations::bitmask::num_ways_n_queens, n);
    benchmark("Optimized Bitmask", implementations::optimized_bitmask::num_ways_n_queens, n);
}
