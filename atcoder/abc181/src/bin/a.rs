use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ans = if n & 1 == 0 { "White" } else { "Black" };

    println!("{}", ans);
}
