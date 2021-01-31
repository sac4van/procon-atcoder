use proconio::*;
use std::cmp::*;

#[fastout]
fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128,
    }

    let ans = if a + b == 0 {
        0
    } else {
        let q = n / (a + b);
        let r = n % (a + b);
        q * a + a.min(r)
    };

    println!("{}", ans);
}
