use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: [isize; 4]
    }
    let (r, c) = (x[2] - x[0], x[3] - x[1]);

    let ans = if r == 0 && c == 0 {
        0
    } else if ((r + c).abs() <= 3 && (r - c).abs() <= 3) || (r == c) || (r == -c) {
        1
    } else if ((r + c).abs() <= 6 && (r - c).abs() <= 6)
        || (r + c).abs() <= 6
        || (r - c).abs() <= 6
        || (r + c) & 1 == 0
    {
        2
    } else {
        3
    };

    println!("{}", ans);
}
