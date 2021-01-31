use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        c: Chars
    }

    let ans = if c[0] == c[1] && c[1] == c[2] {
        "Won"
    } else {
        "Lost"
    };

    println!("{}", ans);
}
