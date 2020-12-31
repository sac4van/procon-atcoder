use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max_gcd_ness = 0;
    let mut ans = 0;

    for k in (2..=1000).rev() {
        let mut gcd_ness = 0;
        for ai in a.iter() {
            if ai % k == 0 {
                gcd_ness += 1;
            }
        }

        if gcd_ness > max_gcd_ness {
            max_gcd_ness = gcd_ness;
            ans = k;
        }
    }

    println!("{}", ans);
}
