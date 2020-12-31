use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; 3]; n],
    }

    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 0..n {
        for j in 0..3 {
            let k = (j + 1) % 3;
            let l = (j + 2) % 3;
            dp[i + 1][j] = max(dp[i][k], dp[i][l]) + a[i][j];
        }
    }
    let ans = max(dp[n][0], max(dp[n][1], dp[n][2]));
    println!("{}", ans);
}
