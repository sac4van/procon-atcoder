use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n == 2 || n & 1 == 1 {
        println!("impossible");
    } else {
    }
}
