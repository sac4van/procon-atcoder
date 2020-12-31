use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        x: isize,
        s: Chars,
    }

    let mut ans = x;

    for c in s {
        ans += if c == 'o' {
            1
        } else if ans > 0 {
            -1
        } else {
            0
        };
    }

    println!("{}", ans);
}
