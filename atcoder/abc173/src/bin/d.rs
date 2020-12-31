use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = a[n - 1];
    let mut cnt = 1;
    for ai in a.iter().rev().skip(1) {
        if cnt < n - 2 {
            cnt += 2;
            ans += 2 * ai;
        } else if cnt == n - 2 {
            cnt += 1;
            ans += ai;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
