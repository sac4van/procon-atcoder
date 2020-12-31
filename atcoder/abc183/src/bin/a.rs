use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize,
    }

    let ans = if x >= 0 { x } else { 0 };

    println!("{}", ans);
}
