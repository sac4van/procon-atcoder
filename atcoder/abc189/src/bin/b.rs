use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }

    let mut drunk = false;
    let mut tot = 0;
    let mut ans: isize = 0;
    for (i, (v, p)) in vp.iter().enumerate() {
        tot += v * p;
        if tot > x * 100 {
            drunk = true;
            ans = (i + 1) as isize;
            break;
        }
    }

    if !drunk {
        ans = -1;
    }

    println!("{}", ans);
}
