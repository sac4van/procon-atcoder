use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    let mut left = 0;
    let mut right = n + 1;
    let mut m = (left + right) / 2;

    while left + 1 != right {
        if m * (m + 1) <= 2 * (n + 1) {
            left = m;
        } else {
            right = m;
        }

        m = (left + right) / 2;
    }

    let ans = n - left + 1;

    println!("{}", ans);
}
