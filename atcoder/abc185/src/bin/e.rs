use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        dp[i][0] = i;
    }

    for j in 1..=m {
        dp[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cand = vec![
                dp[i - 1][j] + 1,
                dp[i][j - 1] + 1,
                dp[i - 1][j - 1] + if a[i - 1] != b[j - 1] { 1 } else { 0 },
            ];
            dp[i][j] = *(cand.iter().min().unwrap());
        }
    }

    println!("{}", dp[n][m]);
}
