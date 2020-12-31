use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        let w1 = wv[i - 1].0;
        let v1 = wv[i - 1].1;
        for j in 0..w1 {
            dp[i][j] = dp[i - 1][j];
        }
        for j in w1..=w {
            dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - w1] + v1);
        }
    }
    let ans = dp[n][w];
    println!("{}", ans);
}
