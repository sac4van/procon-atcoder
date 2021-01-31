use proconio::*;

#[fastout]
fn main() {
    input! {
        x: isize,
        y: isize,
    }

    let ans = if (x - y).abs() < 3 { "Yes" } else { "No" };

    println!("{}", ans);
}
