// Ref: https://www.reddit.com/r/dailyprogrammer/comments/6i60lr/20170619_challenge_320_easy_spiral_ascension/

fn spiral(n: usize) {
    let mut matrix = vec![vec![0; n]; n];
    let mut num = 1;
    let (mut top, mut bottom, mut left, mut right) = (0, n - 1, 0, n - 1);

    while top <= bottom && left <= right {
        for col in left..=right {
            matrix[top][col] = num;
            num += 1;
        }
        top += 1;

        for row in top..=bottom {
            matrix[row][right] = num;
            num += 1;
        }
        if right > 0 {
            right -= 1;
        }

        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom][col] = num;
                num += 1;
            }
            if bottom > 0 {
                bottom -= 1;
            }
        }

        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    let mut print_string = String::new();
    for row in matrix {
        for value in row {
            print_string.push_str(&format!("{:>3} ", value));
        }
        print_string.push('\n');
    }

    println!("{}", print_string);
}

fn main() {
    spiral(6);
}
