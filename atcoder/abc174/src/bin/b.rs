use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        d: i64,
        xy : [(i64,i64); n],
    }

    let mut ans: i64 = 0;

    for &(x, y) in xy.iter() {
        if x*x + y*y <= d*d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
