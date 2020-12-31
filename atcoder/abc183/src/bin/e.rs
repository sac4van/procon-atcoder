use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let _mod = 1000000007 as u128;

    let mut accum_right = vec![vec![0_u128; w + 1]; h + 1];
    let mut accum_down = vec![vec![0_u128; w + 1]; h + 1];
    let mut accum_diag = vec![vec![0_u128; w + 1]; h + 1];
    let mut dp = vec![vec![0_u128; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j - 1] == '.' {
                dp[i][j] = if i == 1 && j == 1 {
                    1
                } else {
                    (accum_right[i - 1][j] + accum_down[i][j - 1] + accum_diag[i - 1][j - 1]) % _mod
                };

                accum_right[i][j] = (accum_right[i - 1][j] + dp[i][j]) % _mod;
                accum_down[i][j] = (accum_down[i][j - 1] + dp[i][j]) % _mod;
                accum_diag[i][j] = (accum_diag[i - 1][j - 1] + dp[i][j]) % _mod;
            } else {
                dp[i][j] = 0;
                accum_right[i][j] = 0;
                accum_down[i][j] = 0;
                accum_diag[i][j] = 0;
            }
        }
    }

    // for i in 1..=h {
    //     for j in 1..=w {
    //         print!("{}, ", dp[i][j]);
    //     }
    //     println!("");
    // }

    println!("{}", dp[h][w]);
}
