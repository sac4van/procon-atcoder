use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut left = 0;
    let mut right = n-1;
    let mut ans = 0;

    while left < right {

        while left < n && c[left] == 'R' {
            left += 1;
        }

        while right > 0 && c[right] == 'W' {
            right -= 1;
        }
    
        if left < right {
            ans += 1;
            left += 1;
            right -= 1;
        }
    }

    println!("{}", ans);
}
