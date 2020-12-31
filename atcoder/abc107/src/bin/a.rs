use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        i: usize,
    }

    println!("{}", n - i + 1);
}
