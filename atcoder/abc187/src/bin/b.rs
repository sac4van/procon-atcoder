use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p : [(isize, isize); n],
    }

    let mut ans = 0;

    for i in 0..n {
        for j in 0..i {
            let (dx, dy) = (p[i].0 - p[j].0, p[i].1 - p[j].1);

            if dx.abs() >= dy.abs() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
