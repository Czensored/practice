// Ref: https://www.reddit.com/r/dailyprogrammer/comments/n3var6/20210503_challenge_388_intermediate_next/
// I don't think that this is a good implementation, but it works
// Not super happy with it

use std::collections::HashMap
fn digits(mut n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result.reverse();
    result
}

fn digits_to_number(digits: &Vec<usize>) -> usize {
    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}

fn nextpal(n: usize) -> usize {
    let n = n + 1;
    let digits = digits(n);
    let len = digits.len();
    let left = &digits[0..len / 2];
    let mut mut_left = left.to_vec();
    
    if len % 2 == 0 {
        let mut reversed: Vec<usize> = mut_left.iter().rev().cloned().collect();
        mut_left.extend(&reversed);

        let num = digits_to_number(&mut_left);

        if num >= n { return num; }

        for i in &mut reversed {
            if *i != 9 {
                *i += 1;
                break;
            }

            *i = 0;
        }

        mut_left = reversed.iter().rev().cloned().collect();
        mut_left.append(&mut reversed);
    } else {
        let mut middle = digits[len / 2];
        let mut reversed: Vec<usize> = mut_left.iter().rev().cloned().collect();
        mut_left.push(middle);
        mut_left.extend(&reversed);

        let num = digits_to_number(&mut_left);

        if num >= n { return num; }

        if middle != 9 { 
            mut_left[len / 2] += 1;
            return digits_to_number(&mut_left);
        }
            
        middle = 0;
        for i in &mut reversed {
            if *i != 9 {
                *i += 1;
                break;
            }

            *i = 0;
        }

        mut_left = reversed.iter().rev().cloned().collect();
        mut_left.push(middle);
        mut_left.append(&mut reversed);
    }
    digits_to_number(&mut_left)
}

fn main() {
    // let n = 3_usize.pow(39);
    let n = 2133;
    println!("{}", nextpal(n));
}
