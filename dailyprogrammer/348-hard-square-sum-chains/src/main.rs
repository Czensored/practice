// Ref: https://www.reddit.com/r/dailyprogrammer/comments/7t6fnc/20180126_challenge_348_hard_square_sum_chains/

use std::collections::HashSet;
mod graph_impl;

fn is_perfect_square(n: u64) -> bool {
    if n < 2 {
        return true;
    }

    let root = (n as f64).sqrt() as u64;
    root * root == n || (root + 1) * (root + 1) == n
}

fn square_sum_dfs_helper(n: usize, path: &mut Vec<usize>, used: &mut HashSet<usize>) -> bool {
    if path.len() == n {
        return true;
    }

    for i in 1..=n {
        if used.contains(&i) {
            continue;
        }

        if path.is_empty() || is_perfect_square((path.last().unwrap() + i) as u64) {
            used.insert(i);
            path.push(i);

            if square_sum_dfs_helper(n, path, used) {
                return true;
            }

            path.pop();
            used.remove(&i);
        }
    }

    false
}

// works well for values of n < 55, too slow after
fn square_sum_dfs(n: usize) -> Option<Vec<usize>> {
    let mut v = Vec::with_capacity(n);
    let mut hs = HashSet::new();
    square_sum_dfs_helper(n, &mut v, &mut hs);
    if v.is_empty() { None } else { Some(v) }
}

fn print_result(r: &Option<Vec<usize>>) {
    if let Some(v) = r {
        println!("{:?}", v);
    } else {
        println!("Not possible");
    }
}

fn main() {
    let n = 40;
    let opt_v = square_sum_dfs(n);
    // let opt_v = graph_impl::square_sum(n);
    print_result(&opt_v);
}
