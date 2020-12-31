use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![1 << 60; n];
    dp[0] = 0;

    for i in 0..n {
        if i + 1 < n {
            dp[i + 1] = min(dp[i + 1], dp[i] + (h[i] - h[i + 1]).abs());
        }
        if i + 2 < n {
            dp[i + 2] = min(dp[i + 2], dp[i] + (h[i + 2] - h[i]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
