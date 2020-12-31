use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize
    }

    fn dfs(tot: usize, i: usize, a: &Vec<usize>, k: usize) -> bool {
        if tot == k {
            true
        } else if (tot > k) || (i >= a.len()) {
            false
        } else {
            dfs(tot + a[i], i + 1, &a, k) || dfs(tot, i + 1, &a, k)
        }
    }

    println!("{}", dfs(0, 0, &a, k));
}
