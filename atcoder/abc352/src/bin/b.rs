#![allow(unused)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    // ===== input =====
    input!{
        s: Chars,
        t: Chars,
    }

    // ===== answer =====
    let mut ans= Vec::<usize>::new();

    // ===== solve =====
    let mut ti = 0;

    for sc in s.iter(){
        for (i,tc) in t[ti..].iter().enumerate() {
            if sc == tc {
                let idx = &ti + &i + 1;
                ti = idx;
                ans.push(idx);
                break;
            }
        }
    }
    // ===== output =====
    for a in &ans{
        print!("{} ", a);
    }
}

