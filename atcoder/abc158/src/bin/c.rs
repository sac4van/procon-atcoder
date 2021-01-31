use proconio::*;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
    }

    // l1 = a/0.08 <= x < (a+1) / 0.08 = u1
    // l2 = b/0.1 <= x < (b+1) / 0.1 = u2

    let (l1, u1) = (
        if 100 * a % 8 == 0 {
            100 * a / 8
        } else {
            (a as f64 / 0.08).ceil() as isize
        },
        if 100 * (a + 1) % 8 == 0 {
            100 * (a + 1) / 8 - 1
        } else {
            ((a as f64 + 1.0) / 0.08).floor() as isize
        },
    );
    let (l2, u2) = (10 * b, 10 * (b + 1) - 1);

    let l = l1.max(l2);
    let u = u1.min(u2);

    let ans = if l <= u { l } else { -1 };

    println!("{}", ans);
}
