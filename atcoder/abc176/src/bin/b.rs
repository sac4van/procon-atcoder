use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let mut s = 0;

    for d in n {
        s += (d as usize) - ('0' as usize);
    }

    let ans = if s % 9 == 0 { "Yes" } else { "No" };

    println!("{}", ans);
}
