use proconio::marker::*;
use proconio::*;
use std::cmp::*;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = t.len();
    for i in 0..(1 + s.len() - t.len()) {
        let mut num_same_chars = 0;
        for j in 0..(t.len()) {
            num_same_chars += if s[i + j] == t[j] { 1 } else { 0 };
        }

        ans = min(ans, t.len() - num_same_chars);
    }

    println!("{}", ans);
}
