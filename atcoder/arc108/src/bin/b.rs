use proconio::marker::Chars;
use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans_str: Vec<char> = Vec::new();

    for c in s {
        if c == 'x' {
            let m = ans_str.len();
            if m >= 2 && ans_str[m - 1] == 'o' && ans_str[m - 2] == 'f' {
                ans_str.pop();
                ans_str.pop();
                continue;
            }
        }
        ans_str.push(c);
    }

    println!("{}", ans_str.len());
}
