// Ref: https://www.reddit.com/r/dailyprogrammer/comments/oirb5v/20210712_challenge_398_difficult_matrix_sum/
// Ref: https://en.wikipedia.org/wiki/Hungarian_algorithm
// Ref: https://users.cs.duke.edu/%7Ebrd/Teaching/Bio/asmb/current/Handouts/munkres.html

use num_traits::{Bounded, Signed, Zero};
use reqwest::blocking;
use std::error::Error;
use std::ops::{Add, Neg, Sub};

pub trait CostType:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Zero
    + Bounded
    + Signed
{
}

impl<T> CostType for T where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Zero
        + Bounded
        + Signed
{
}

fn ckmin<T: PartialOrd + Copy>(a: &mut T, b: T) -> bool {
    if b < *a {
        *a = b;
        true
    } else {
        false
    }
}

fn hungarian<T: CostType>(c: &Vec<Vec<T>>) -> Vec<T> {
    let j = c.len();
    let w = c[0].len();
    assert!(j <= w);

    let mut job = vec![usize::MAX; w + 1];
    let mut ys = vec![T::zero(); j];
    let mut yt = vec![T::zero(); w + 1];
    let mut answers = Vec::with_capacity(j);

    let inf = T::max_value();

    for j_cur in 0..j {
        let mut w_cur = w;
        job[w_cur] = j_cur;

        let mut min_to = vec![inf; w + 1];
        let mut prev = vec![usize::MAX; w + 1];
        let mut in_z = vec![false; w + 1];

        while job[w_cur] != usize::MAX {
            in_z[w_cur] = true;
            let j = job[w_cur];
            let mut delta = inf;
            let mut w_next = 0;

            for w1 in 0..w {
                if !in_z[w1] {
                    let reduced = c[j][w1] - ys[j] - yt[w1];
                    if ckmin(&mut min_to[w1], reduced) {
                        prev[w1] = w_cur;
                    }
                    if ckmin(&mut delta, min_to[w1]) {
                        w_next = w1;
                    }
                }
            }

            for w1 in 0..=w {
                if in_z[w1] {
                    ys[job[w1]] = ys[job[w1]] + delta;
                    yt[w1] = yt[w1] - delta;
                } else {
                    min_to[w1] = min_to[w1] - delta;
                }
            }

            w_cur = w_next;
        }

        while w_cur != w {
            let w1 = prev[w_cur];
            job[w_cur] = job[w1];
            w_cur = w1;
        }

        answers.push(-yt[w]);
    }

    answers
}

#[allow(dead_code)]
fn sanity_check_hungarian() {
    let costs = vec![vec![8, 5, 9], vec![4, 2, 4], vec![7, 3, 8]];

    let expected = vec![5, 9, 15];
    let result = hungarian(&costs);

    assert_eq!(result, expected);
    eprintln!("Sanity check passed.");
}

fn min_cost<T: CostType>(costs: &Vec<Vec<T>>) -> T {
    let potentials = hungarian(&costs);
    *potentials
        .last()
        .expect("Hungarian output should not be empty")
}

#[allow(dead_code)]
fn test1() {
    let costs = vec![
        vec![123456789, 752880530, 826085747, 576968456, 721429729],
        vec![173957326, 1031077599, 407299684, 67656429, 96549194],
        vec![1048156299, 663035648, 604085049, 1017819398, 325233271],
        vec![942914780, 664359365, 770319362, 52838563, 720059384],
        vec![472459921, 662187582, 163882767, 987977812, 394465693],
    ];

    let result = min_cost(&costs);
    println!("{}", result);
}

fn test_from_url(url: &str) -> Result<(), Box<dyn Error>> {
    let response = blocking::get(url)?.text()?;

    let costs: Vec<Vec<i64>> = response
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let result = min_cost(&costs);
    println!("{}", result);
    Ok(())
}

fn main() {
    // sanity_check_hungarian();
    // test1();
    // let url = "https://gist.githubusercontent.com/cosmologicon/4f6473b4e781f20d4bdef799132a3b4b/raw/d518a7515618f70d25c2bc6c58430f732f6e06ce/matrix-sum-20.txt";
    // let _ = test_from_url(url);
    let url = "https://gist.githubusercontent.com/cosmologicon/641583595e2c76d7c119912f7afafbfe/raw/6f9ebcb354c3aa58fb19c6f4208d0eced310b62a/matrix-sum-97.txt";
    let _ = test_from_url(url);
}
