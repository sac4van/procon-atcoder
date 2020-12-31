use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }
    let v_max = 100001;
    let mut dp = vec![vec![1 << 60; v_max]; n + 1];
    // dp[n][v]
    //  items: 1,2,...,n
    //  minimum total weight with which the total value equals v
    // dp[n][v] = min(dp[n-1][v], dp[n-1][v - vi] + wi)
    //
    let mut ans = 0;

    dp[0][0] = 0;

    for i in 1..=n {
        let w1 = wv[i - 1].0;
        let v1 = wv[i - 1].1;
        for j in 0..v1 {
            dp[i][j] = dp[i - 1][j];
        }
        for j in v1..v_max {
            dp[i][j] = min(dp[i - 1][j], dp[i - 1][j - v1] + w1);
        }
        for j in 0..v_max {
            if dp[i][j] <= w {
                ans = max(ans, j);
            }
        }
    }
    println!("{}", ans);
}
