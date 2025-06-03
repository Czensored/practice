// Ref: https://www.reddit.com/r/dailyprogrammer/comments/o4uyzl/20210621_challenge_395_easy_nonogram_row/

fn nonogram_row(v: &Vec<usize>) -> Vec<usize> {
    let mut ret_vec = Vec::new();
    let mut ones_streak = 0;

    for i in v {
        if *i != 0 {
            ones_streak += 1;
        } else {
            if ones_streak != 0 { ret_vec.push(ones_streak); }
            ones_streak = 0;
        }
    }
    if ones_streak != 0 { ret_vec.push(ones_streak); }

    if ret_vec.is_empty() { vec![0] } else { ret_vec }
}

fn main() {
    println!("{:?}", nonogram_row(&vec![])); // [0]
    println!("{:?}", nonogram_row(&vec![0, 0, 0, 0, 0])); // [0]
    println!("{:?}", nonogram_row(&vec![1, 1, 1, 1, 1])); // [5]
    println!("{:?}", nonogram_row(&vec![0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1])); // [5,4]
    println!("{:?}", nonogram_row(&vec![1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0])); // [2,1,3]
    println!("{:?}", nonogram_row(&vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1])); // [2,1,3]
    println!("{:?}", nonogram_row(&vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1])); // [1,1,1,1,1,1,1,1]
}
