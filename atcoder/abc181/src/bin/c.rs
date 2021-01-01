use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(isize, isize); n],
    }

    let mut ok = false;

    for i in (0..n).combinations(3) {
        let (dx1, dy1) = (p[i[1]].0 - p[i[0]].0, p[i[1]].1 - p[i[0]].1);
        let (dx2, dy2) = (p[i[2]].0 - p[i[0]].0, p[i[2]].1 - p[i[0]].1);

        if dx1 * dy2 == dx2 * dy1 {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
