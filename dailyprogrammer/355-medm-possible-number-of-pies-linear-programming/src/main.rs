// Ref: https://www.reddit.com/r/dailyprogrammer/comments/87rz8c/20180328_challenge_355_intermediate_possible/

// Solution using linear programming
fn num_pies(v: &Vec<usize>) -> (usize, usize) {
    assert_eq!(v.len(), 5, "vec should have a length of 5");

    let mut best_val = 0;
    let mut best_pump = 0;
    let mut best_appl = 0;

    for pumpkin in 0..=v[0] {
        for apple in 0..=v[1] {
            // Constraints
            if 3 * pumpkin + 4 * apple > v[2] { continue; }
            if 4 * pumpkin + 3 * apple > v[3] { continue; }
            if 3 * pumpkin + 2 * apple > v[4] { continue; }

            // Can weight differently if you like a certain type of pie more
            let val = pumpkin + apple;
            if val > best_val {
                best_val = val;
                best_pump = pumpkin;
                best_appl = apple;
            }
        }
    }
    (best_pump, best_appl)
}

fn main() {
    println!("{:?}", num_pies(&vec![10usize, 14, 10, 42, 24]));
    println!("{:?}", num_pies(&vec![12usize, 4, 40, 30, 40]));
    println!("{:?}", num_pies(&vec![12usize, 14, 20, 42, 24]));
}
