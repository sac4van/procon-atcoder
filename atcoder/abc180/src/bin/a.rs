use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }

    let ans = n - a + b;

    println!("{}", ans);
}
