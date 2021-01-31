use proconio::*;

fn mod_exp(base: usize, exponent: usize, modulo: usize) -> usize {
    if exponent == 0 {
        return 1;
    }
    if exponent == 1 {
        return base % modulo;
    }

    let mut t = mod_exp(base, exponent >> 1, modulo);
    t = (t * t) % modulo;

    if exponent & 1 == 0 {
        return t;
    } else {
        return ((base % modulo) * t) % modulo;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let r2 = mod_exp(10, n, m * m);
    let ans = r2 / m;
    println!("{}", ans);
}
