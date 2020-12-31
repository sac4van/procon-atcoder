use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut ans = s;

    println!("{}", ans);
}
