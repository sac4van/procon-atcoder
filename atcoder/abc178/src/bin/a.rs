use proconio::*;

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    let ans = 1 - x;

    println!("{}", ans);
}
