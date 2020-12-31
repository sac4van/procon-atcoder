use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    };
    let mut ans = vec![0; 4];
    for t in s.iter() {
        let idx = match t.as_str() {
            "AC" => 0,
            "WA" => 1,
            "TLE" => 2,
            "RE" => 3,
            _ => panic!(),
        };
        ans[idx] += 1;
    }
    println!("AC x {}", ans[0]);
    println!("WA x {}", ans[1]);
    println!("TLE x {}", ans[2]);
    println!("RE x {}", ans[3]);
}
