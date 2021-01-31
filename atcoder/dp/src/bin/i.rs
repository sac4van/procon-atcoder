use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    // dp[i][j] = Prob[# of head = j, when i coins tossed.]
    let mut dp = vec![vec![0_f64; n + 1]; n + 1];
    dp[0][0] = 1_f64;

    for i in 1..=n {
        dp[i][0] = (1.0 - p[i - 1]) * dp[i - 1][0];
        for j in 1..=i {
            dp[i][j] = p[i - 1] * dp[i - 1][j - 1] + (1.0 - p[i - 1]) * dp[i - 1][j];
        }
    }

    let mut ans = 0_f64;
    for j in (n / 2 + 1)..=n {
        ans += dp[n][j];
    }

    println!("{}", ans);
}
