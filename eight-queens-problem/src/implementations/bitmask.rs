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

    solve(0, n, 0, 0, 0)
}

#[allow(dead_code)]
fn main() {
    let n = 15;
    let num_ways = num_ways_n_queens(n);
    println!("{}", num_ways);
}