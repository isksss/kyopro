use proconio::input;

fn main(){
    input!{
        s: String
    }

    let mut cnt = [0;26];

    s.chars().for_each(|x| {
        cnt[((x as i32) - ('a' as i32)) as usize] += 1;
    });

    let mut freqs = [0; 101];
    for c in cnt {
        if c > 0 {
            freqs[c] += 1;
        }
    }

    let mut flg = true;
    for f in freqs {
        if f != 0 && f != 2 {
            flg = false;
            break;
        }
    }

    println!("{}", if flg { "Yes" } else { "No" });
}
