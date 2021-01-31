use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set1: HashSet<String> = HashSet::new();
    let mut set2: HashSet<String> = HashSet::new();
    for si in s {
        if si.chars().collect::<Vec<char>>()[0] == '!' {
            let t: String = String::from(&si[1..(si.len())]);
            set1.insert(t);
        } else {
            set2.insert(si);
        }
    }

    let mut satisfiable = true;
    for t in set1 {
        if set2.contains(&t) {
            satisfiable = false;
            println!("{}", t);
            break;
        }
    }

    if satisfiable {
        println!("satisfiable");
    }
}
