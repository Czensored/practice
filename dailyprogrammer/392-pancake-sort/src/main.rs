// Ref: https://www.reddit.com/r/dailyprogrammer/comments/np3sio/20210531_challenge_392_intermediate_pancake_sort/

fn flip_front(v: &mut Vec<isize>, n: usize) {
    assert!(n > 1, "n is too small");
    assert!(n <= v.len(), "n is too large");

    v[..n].reverse();
}

fn pancake_sort(v: &mut Vec<isize>) {
    while !v.is_sorted() {
    }
}

fn main() {
    // let mut v = vec![0, 1, 2, 3, 4];
    // flip_front(&mut v, 2);
    // println!("{:?}", v);
    let mut v = vec![3, 1, 2, 1];
    pancake_sort(&mut v);
    println!("{:?}", v);
}
