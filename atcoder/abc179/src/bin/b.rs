use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }

    let mut ok: bool = true;

    for i in 0..(n - 2) {
        ok = true;
        for j in 0..3 {
            if d[i + j].0 != d[i + j].1 {
                ok = false;
                break;
            }
        }

        if ok {
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
