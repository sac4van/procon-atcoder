use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [f64; n]
    }

    let mut dm = 0_f64;
    let mut de = 0_f64;
    let mut dc = 0_f64;

    for xi in x {
        dm += xi.abs();
        de += xi * xi;
        dc = if dc > xi.abs() { dc } else { xi.abs() };
    }

    de = de.sqrt();

    println!("{}\n{}\n{}", dm, de, dc);
}
