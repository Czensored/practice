use st_petersburg_paradox::*;

fn main() {
    println!("Hello!");
    let trials = 1000000;
    // let avg = rolling_avg_paralell(trials);
    let avg = rolling_avg(trials);
    println!("Average over {trials} trials: {avg}");
}
