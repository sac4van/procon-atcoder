use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        q: [(i128, i128, i128); t],
    }

    // solve a*x + b*y = gcd(a,b)
    // returns (gcd(a,b), x, y)
    fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b != 0 {
            let (d, x, y) = egcd(b, a % b);
            (d, y, x - (a / b) * y)
        } else {
            (a, 1, 0)
        }
    }

    for (n, s, k) in q {
        let (g, _, q) = egcd(n, k);

        // choose q the negative with minimum abs.
        // p := p + m * (k/g)
        // q := q - m * (n/g)

        println!(
            "{}",
            if s % g == 0 {
                let (_n, _k, _s) = (n / g, k / g, s / g);

                if q < 0 {
                    (-q * _s) % _n
                } else {
                    _n - (q * _s) % _n
                }
            } else {
                -1
            }
        );
    }
}
