#![allow(unused)]
use proconio::input;

fn main() {
    // ===== input =====
    input!{
        x: isize,
        y: isize,
        mut n: isize,
    }

    // ===== answer =====
    let mut ans: isize = 0;

    // ===== solve =====
    // println!("x: {}, y:{}, n:{}", x, y, n);
    if x*3 < y{
        ans = n*x;
        n = -1;
    }
    loop{
        // println!("n:{}, ans:{}", n, ans);

        if n-3 >= 0 {
            ans = ans + y;
            n = n-3;
        }else{
            if n-1 >= 0{
                ans = ans + x;
                n = n - 1;
            }else{
                break;
            }
        }
    }

    // ===== output =====
    println!("{}", ans);
}
