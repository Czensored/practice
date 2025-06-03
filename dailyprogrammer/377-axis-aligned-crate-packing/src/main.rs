// Ref: https://www.reddit.com/r/dailyprogrammer/comments/bazy5j/20190408_challenge_377_easy_axisaligned_crate/

use itertools::Itertools;
use std::cmp::max;

fn fit1(x_main: usize, y_main: usize, x_1: usize, y_1: usize) -> usize {
    (x_main / x_1) * (y_main / y_1)
}

fn fit2(x_main: usize, y_main: usize, x_1: usize, y_1: usize) -> usize {
    max(
        fit1(x_main, y_main, x_1, y_1),
        fit1(x_main, y_main, y_1, x_1),
    )
}

fn fitn(vec_main: Vec<usize>, vec_1: Vec<usize>) -> usize {
    if vec_main.len() != vec_1.len() {
        return 0;
    }
    let mut max_val = 0;
    for perm in vec_1.iter().permutations(vec_1.len()) {
        let mut count = 1;
        for i in 0..vec_main.len() {
            count *= vec_main[i] / perm[i];
        }
        max_val = max(max_val, count);
    }
    max_val
}

fn main() {
    println!("{}", fit1(12345, 678910, 1112, 1314)); // 5676
    println!("{}", fit2(25, 18, 6, 5)); // 15
    println!("{}", fitn(vec![123, 456, 789], vec![10, 11, 12])); // 32604
    println!(
        "{}",
        fitn(
            vec![123, 456, 789, 1011, 1213, 1415],
            vec![16, 17, 18, 19, 20, 21]
        )
    ); // 1883443968
}
