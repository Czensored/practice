pub fn num_ways_n_queens(n: usize) -> usize {
    let mut cols = vec![false; n];
    let mut main_diag = vec![false; 2 * n];
    let mut anti_diag = vec![false; 2 * n];

    fn solve(
        row: usize,
        n: usize,
        cols: &mut Vec<bool>,
        main_diag: &mut Vec<bool>,
        anti_diag: &mut Vec<bool>
    ) -> usize {
        if row == n { return 1; }

        let mut result = 0;
        
        for col in 0..n {
            let md = row + col;
            let ad = row + n - col;

            if cols[col] || main_diag[md] || anti_diag[ad] { continue; }

            cols[col] = true;
            main_diag[md] = true;
            anti_diag[ad] = true;
            result += solve(row + 1, n, cols, main_diag, anti_diag);
            cols[col] = false;
            main_diag[md] = false;
            anti_diag[ad] = false;
        }

        result
    }

    solve(0, n, &mut cols, &mut main_diag, &mut anti_diag)
}

#[allow(dead_code)]
fn main() {
    let n = 8;
    let num_ways = num_ways_n_queens(n);
    println!("{}", num_ways);
}