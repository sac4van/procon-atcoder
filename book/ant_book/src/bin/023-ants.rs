use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        l: usize,
        n: usize,
        x: [usize; n]
    }

    let mut ans_min = 0;
    let mut ans_max = 0;

    for &xi in x.iter() {
        ans_min = max(ans_min, min(xi, l - xi));
        ans_max = max(ans_max, max(xi, l - xi));
    }

    println!("min: {}, max: {}", ans_min, ans_max);
}
