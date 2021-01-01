use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let int_max = 1000001;

    // d[x]: the minimum prime that divides x
    let mut d = vec![0; int_max];
    for i in 2..int_max {
        d[i] = i;
    }
    for i in 2..int_max {
        if d[i] == i {
            let mut j = 2 * i;
            while j < int_max {
                d[j] = i;
                j += i
            }
        }
    }

    let prime_factorization = |k: usize| -> HashSet<usize> {
        let mut set: HashSet<usize> = HashSet::new();
        let mut _k = k;
        loop {
            if _k <= 1 {
                break set;
            }
            set.insert(d[_k]);
            _k /= d[_k];
        }
    };

    let mut primes: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut used_primes: HashSet<usize> = HashSet::new();
    let mut is_pairwise_prime = true;
    let mut id_with_min_plen = 0;

    for i in 0..n {
        primes[i] = prime_factorization(a[i]);
        if primes[i].len() > 0 && primes[i].len() < primes[id_with_min_plen].len() {
            id_with_min_plen = i;
        }

        for &p in &primes[i] {
            if used_primes.contains(&p) {
                is_pairwise_prime = false;
            } else {
                used_primes.insert(p);
            }
        }
    }

    let mut is_setwise_prime = true;
    for &p in &primes[id_with_min_plen] {
        let mut exists_common_divisor = true;
        for i in 0..n {
            if !primes[i].contains(&p) {
                exists_common_divisor = false;
                break;
            }
        }

        if exists_common_divisor {
            is_setwise_prime = false;
        }
    }

    let ans = if is_pairwise_prime {
        "pairwise coprime"
    } else if is_setwise_prime {
        "setwise coprime"
    } else {
        "not coprime"
    };

    println!("{}", ans);
}
