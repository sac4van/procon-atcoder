use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }

    let ans = (10000-n) % 1000;

    println!("{}", ans);
}
