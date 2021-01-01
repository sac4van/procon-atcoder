use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    let mut que1: VecDeque<u128> = VecDeque::new();
    let mut que2: VecDeque<u128> = VecDeque::new();

    let mut i = 1;
    loop {
        if i * i > n {
            break;
        } else if i * i == n {
            que1.push_back(i);
            break;
        }

        if n % i == 0 {
            que1.push_back(i);
            que2.push_front(n / i);
        }

        i += 1;
    }

    que1.append(&mut que2);

    for ans in que1 {
        println!("{}", ans);
    }
}
