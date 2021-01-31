use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut dp = vec![vec![0; w + 1]; h + 1];
    let modulo = 1000000007;

    for i in 1..=h {
        for j in 1..=w {
            if i == 1 && j == 1 {
                dp[i][j] = 1;
            } else {
                let f = a[i - 1][j - 1];
                dp[i][j] = if f == '.' {
                    (dp[i - 1][j] + dp[i][j - 1]) % modulo
                } else {
                    0
                };
            }
        }
    }

    println!("{}", dp[h][w]);
}
