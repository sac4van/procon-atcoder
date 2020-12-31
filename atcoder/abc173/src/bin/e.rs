use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }

    let _mod = isize::pow(10, 9) + 7;

    let mut num_zero = 0;
    let mut a_plus: Vec<isize> = Vec::new();
    let mut a_minus: Vec<isize> = Vec::new();

    let mut ans = 0;

    for &ai in a.iter() {
        if ai > 0 {
            a_plus.push(ai);
        } else if ai < 0 {
            a_minus.push(-ai);
        } else {
            num_zero += 1;
        }
    }
    a_plus.sort();
    a_minus.sort();

    if a_plus.len() + a_minus.len() < k {
        ans = 0;
    } else {
    }

    println!("{}", ans);
}
