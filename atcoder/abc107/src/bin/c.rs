use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: [isize; n],
    }

    let mut ans = 1 << 60;
    let mut left = 0;
    let mut right = k - 1;

    while right < n {
        ans = min(ans, x[right]-x[left] + min(x[left].abs(), x[right].abs()));
        left += 1;
        right += 1;
    }

    println!("{}", ans);
}
