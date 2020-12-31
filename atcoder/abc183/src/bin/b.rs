use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }

    let ans = (sx * gy + sy * gx) / (sy + gy);

    println!("{}", ans);
}
