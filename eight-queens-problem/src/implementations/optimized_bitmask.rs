// NOT MY IMPLEMTNETAION

pub fn num_ways_n_queens(n: usize) -> usize {
    fn solve(
        row: usize,
        n: usize,
        cols: usize,
        main_diags: usize,
        anti_diags: usize,
    ) -> usize {
        if row == n { return 1; }

        let mut result = 0;

        let full = (1 << n) - 1;
        let attacked = cols | main_diags | anti_diags;
        let mut avail = full & !attacked;

        while avail != 0 {
            let col_bit = avail & avail.wrapping_neg();
            avail ^= col_bit;

            result += solve(
                row + 1, n, cols | col_bit, 
                (main_diags | col_bit) << 1,
                (anti_diags | col_bit) >> 1,
            )
        }
        result
    }

    let mut result = 0;

    for col in 0..(n / 2) {
        let col_mask = 1 << col;
        let main_diag = col_mask << 1;
        let anti_diag = col_mask >> 1;

        result += solve(1, n, col_mask, main_diag, anti_diag);
    }

    result *= 2;

    if n % 2 == 1 {
        let mid_col = n / 2;
        let col_mask = 1 << mid_col;
        let main_diag = col_mask << 1;
        let anti_diag = col_mask >> 1;

        result += solve(1, n, col_mask, main_diag, anti_diag);
    }

    result
}

#[allow(dead_code)]
fn main() {
    let n = 18;
    let num_ways = num_ways_n_queens(n);
    println!("{}", num_ways);
}
