use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize,usize); n]
    }

    let mut ans = 0;

    ab.sort_by_key(|p| p.0);

    let mut cans = 0;
    for &x in ab.iter() {
        if cans + x.1 >= m {
            ans += (m - cans) * x.0;
            break;
        } else {
            ans += x.0 * x.1;
            cans += x.1;
        }
    }

    println!("{}", ans);
}
