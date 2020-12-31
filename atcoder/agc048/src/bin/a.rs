use proconio::marker::Chars;
use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        s_list: [Chars; n],
    }

    let atcoder: Vec<char> = "atcoder".chars().collect();

    for s in s_list.iter() {
        let mut _first = 2000;
        for _i in 0..(s.len()) {
            if s[_i] != 'a' {
                _first = _i;
                break;
            }
        }

        let mut _ans: isize = 0;
        if _first == 0 {
            _ans = 0;
        } else if _first == 2000 {
            _ans = -1;
        } else {
            if s[_first] < 't' {
                _ans = _first as isize;
            } else if s[_first] > 't' {
                _ans = _first as isize - 1;
            } else {
                let mut _greater_than_atcoder = false;
                for _i in 0..(s.len()) {
                    if s[_i] > atcoder[_i] {
                        _greater_than_atcoder = true;
                        break;
                    } else if s[_i] < atcoder[_i] {
                        break;
                    }

                    if _i == 6 {
                        if s.len() > 7 {
                            _greater_than_atcoder = true;
                        }
                        break;
                    }
                }
                if _greater_than_atcoder {
                    _ans = 0;
                } else {
                    if _first == 1 {
                        _ans = 1;
                    } else {
                        _ans = _first as isize;
                    }
                }
            }
        }
        println!("{}", _ans);
    }
}
