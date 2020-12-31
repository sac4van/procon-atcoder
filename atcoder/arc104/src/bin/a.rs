use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize
    }

    println!("{} {}", (a + b) / 2, (a - b) / 2);
}
