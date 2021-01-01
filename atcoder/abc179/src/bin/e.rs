use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: u128,
        x: u128,
        m: u128,
    }

    let mut root: u128 = 1;
    let mut sum_cycle: u128 = 0;
    let mut len_cycle: u128 = 0;

    let mut set: HashSet<u128> = HashSet::new();
    let mut len_1: u128 = 0;
    let mut sum_1: u128 = 0;
    let mut ai = x;

    let mut finished = false;

    // find root
    loop {
        if set.contains(&ai) {
            root = ai;
            break;
        }
        set.insert(ai);
        len_1 += 1;
        sum_1 += ai;
        ai = (ai * ai) % m;

        if len_1 == n {
            finished = true;
            break;
        }
    }

    let mut ans: u128 = sum_1;

    if !finished {
        // calc loop
        ai = root;
        set.clear();
        while !set.contains(&ai) {
            set.insert(ai);
            len_cycle += 1;
            sum_cycle += ai;
            ai = (ai * ai) % m;
        }

        ans += (n - len_1) / len_cycle * sum_cycle;

        // rest
        let len_2 = (n - len_1) % len_cycle;
        ai = root;
        for _ in 0..len_2 {
            ans += ai;
            ai = (ai * ai) % m;
        }
    }

    println!("{}", ans);
}
