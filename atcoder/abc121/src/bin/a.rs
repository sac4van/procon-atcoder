use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        h: usize,
        w: usize

    }

    let ans = (H - h) * (W - w);

    println!("{}", ans);
}
