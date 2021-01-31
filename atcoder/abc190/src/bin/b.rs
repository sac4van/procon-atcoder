use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n],
    }

    let mut ok = false;

    for (x, y) in xy {
        if x < s && y > d {
            ok = true;
            break;
        }
    }

    let ans = if ok { "Yes" } else { "No" };

    println!("{}", ans);
}
