use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(i128, i128); n],
    }

    let mut d: Vec<i128> = Vec::new();

    let mut sum = 0;
    for (a, b) in ab {
        sum += a;
        d.push(2 * a + b);
    }

    d.sort();

    let mut ans = 0;

    while sum >= 0 {
        ans += 1;
        sum -= d.pop().unwrap();
    }

    println!("{}", ans);
}
