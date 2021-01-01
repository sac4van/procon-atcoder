use proconio::*;

// solve a*x + b*y = gcd(a,b)
// returns (gcd(a,b), x, y)
#[allow(dead_code)]
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b != 0 {
        let (d, x, y) = egcd(b, a % b);
        (d, y, x - (a / b) * y)
    } else {
        (a, 1, 0)
    }
}

#[allow(dead_code)]
fn inv(a: i128, m: i128) -> i128 {
    if m <= 0 {
        panic!("m must be a positive integer. [m={}]", m);
    }
    let (_, x, _) = egcd(a, m);
    let mut ret = x;
    loop {
        if ret > 0 {
            break ret;
        }
        ret += m;
    }
}

#[fastout]
fn main() {
    input! {
        s: i128,
    }
    let m = 1000000000 + 7;
    let mut ans = 0;

    for i in 1..=(s / 3) {
        let mut r = 1;
        for j in 1..i {
            r = (r * (s - 2 * i - j) * inv(j, m)) % m;
        }
        ans = (ans + r) % m;
    }

    println!("{}", ans);
}
