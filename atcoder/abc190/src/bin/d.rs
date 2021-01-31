use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // 2N = (a+b)* (a-b+1),  a >= b

    // 2N = p*q
    // a = (p+q-1)/2,
    // b = (p-q+1)/2
    //    a >= b  =>  q >= 1

    let mut ans = 0;

    let mut q = 1;
    while q * q <= 2 * n {
        if 2 * n % q == 0 {
            let p = 2 * n / q;
            if (p + q) & 1 == 1 {
                // let a = (p + q - 1) / 2;
                // let b = (p - q + 1) / 2;
                ans += 2;
            }
        }
        q += 1;
    }

    println!("{}", ans);
}
