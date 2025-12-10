use st_petersburg_paradox::*;

fn main() {
    let trials = 1_000_000_000;
    let avg = rolling_avg_parallel(trials);
    // let avg = rolling_avg(trials);
    println!("Average over {} trials: {}", trials, avg);
}
