use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::max;
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ns = s.len();
    let nt = t.len();

    // max len of common sequences s[0..i] and t[0..j]
    let mut dp = vec![vec![0; nt + 1]; ns + 1];

    for i in 0..ns {
        for j in 0..nt {
            dp[i + 1][j + 1] = if s[i] == t[j] {
                dp[i][j] + 1
            } else {
                max(dp[i + 1][j], dp[i][j + 1])
            }
        }
    }

    // construct data
    let mut lcs = Vec::new();
    let mut i = ns;
    let mut j = nt;
    let mut l = dp[i][j];

    loop {
        if l == 0 {
            break;
        }

        if s[i - 1] == t[j - 1] {
            lcs.push(s[i - 1]);
            i -= 1;
            j -= 1;
            l -= 1;
        } else if dp[i - 1][j] == l {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    let ans = lcs.iter().rev().collect::<String>();

    println!("{}", ans);
}
