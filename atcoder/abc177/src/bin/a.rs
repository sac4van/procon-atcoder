use proconio::*;

#[fastout]
fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize,
    }

    let ans = if d <= t * s { "Yes" } else { "No" };

    println!("{}", ans);
}
