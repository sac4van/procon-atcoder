use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: i64,
    }

    let mut ans: i64 = -1;
    let mut count = 0;
    let mut a = 0;

    while count <= k {
        let b = (10 * a + 7) % k;
        count += 1;

        if b == 0 {
            ans = count;
            break;
        } 
        
        a = b;
    }
    println!("{}", ans);
}
