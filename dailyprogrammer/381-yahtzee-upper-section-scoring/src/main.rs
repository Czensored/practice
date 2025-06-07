// Ref: https://www.reddit.com/r/dailyprogrammer/comments/dv0231/20191111_challenge_381_easy_yahtzee_upper_section/

use std::collections::HashMap;

fn yahtzee_upper(input: Vec<usize>) -> usize {
    let mut scores = HashMap::new();
    let mut max = 0;

    for i in input {
        let count = scores.entry(i).or_insert(0);
        *count += 1;
        let total = i * *count;
        if total > max {
            max = total;
        }
    }

    max
}

fn main() {
    println!("{}", yahtzee_upper(vec![2, 3, 5, 5, 6])); // 10
    println!("{}", yahtzee_upper(vec![1, 1, 1, 1, 3])); // 4
    println!("{}", yahtzee_upper(vec![1, 1, 1, 3, 3])); // 6
    println!("{}", yahtzee_upper(vec![1, 2, 3, 4, 5])); // 5
    println!("{}", yahtzee_upper(vec![6, 6, 6, 6, 6])); // 30
    println!("{}", yahtzee_upper(vec![
            1654, 1654, 50995, 30864, 1654, 50995, 22747, 1654, 1654, 1654, 1654, 1654, 30864,
            4868, 1654, 4868, 1654, 30864, 4868, 30864
        ])); // 123456
}
