use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize
    }

    let ans = t * ((n + x - 1) / x);

    println!("{}", ans);
}
