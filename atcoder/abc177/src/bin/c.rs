use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u128; n],
    }

    let m = 1000000000 + 7;
    let mut s1: u128 = 0;
    let mut s2: u128 = 0;

    for ai in a {
        s1 += ai;
        s2 += ai * ai;
    }

    let ans = (s1 * s1 - s2) / 2 % m;

    println!("{}", ans);
}
