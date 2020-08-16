use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        mut l: [i64; n],
    }

    let mut ans = 0;

    for (i, x) in l.iter().enumerate() {
        for (j, y) in l.iter().enumerate().take(i) {
            for z in l.iter().take(j) {
                if *x == *y || *y == *z || *z == *x {
                    continue;
                } else if *x + *y > *z && *y + *z > *x && *z + *x > *y {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
