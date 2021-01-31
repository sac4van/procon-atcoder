use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; 1 << n],
    }

    let mut left_max = 0;
    let mut left_idx = 0;
    let mut right_max = 0;
    let mut right_idx = 0;
    let size = 1 << n;

    for i in 0..(size / 2) {
        if left_max < a[i] {
            left_max = a[i];
            left_idx = i;
        }
        if right_max < a[i + size / 2] {
            right_max = a[i + size / 2];
            right_idx = i + size / 2;
        }
    }

    let ans = if left_max < right_max {
        left_idx + 1
    } else {
        right_idx + 1
    };

    println!("{}", ans);
}
