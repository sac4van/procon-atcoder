use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    fn contains7(n: usize, base: usize) -> bool {
        let mut _n = n;
        while _n > 0 {
            if _n % base == 7 {
                return true;
            }
            _n /= base;
        }
        false
    }
    for i in 1..=n {
        if !(contains7(i, 8) || contains7(i, 10)) {
            ans += 1
        }
    }

    println!("{}", ans);
}
