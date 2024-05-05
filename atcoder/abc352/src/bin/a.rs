#![allow(unused)]
use proconio::input;

fn main() {
    // ===== input =====
    input!{
        _n: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    // ===== answer =====
    let mut ans: &str = "No";

    // ===== solve =====
    if (x<y && x<z && z<y) || (x>y && y<z && z<x){
        ans = "Yes";
    }

    // ===== output =====
    println!("{}", ans);
}

