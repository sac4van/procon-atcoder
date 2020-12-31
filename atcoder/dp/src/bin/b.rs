use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }

    let mut dp = vec![1 << 60; n];
    dp[0] = 0;

    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                dp[i + j] = min(dp[i + j], dp[i] + (h[i + j] - h[i]).abs())
            }
        }
    }

    println!("{}", dp[n - 1]);
}
