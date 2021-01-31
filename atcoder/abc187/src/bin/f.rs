use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        s: String,
        n: usize,
        mut l: [i64; n],
    }

    let mut ans = s;

    println!("{}", ans);
}
