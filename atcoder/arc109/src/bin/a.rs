use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        x: isize,
        y: isize,
    }

    let ans = if a > b {
        x + (a - b - 1).abs() * min(2 * x, y)
    } else {
        x + (a - b).abs() * min(2 * x, y)
    };

    println!("{}", ans);
}
