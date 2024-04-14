use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        mut a: u32, // フォロワー
        mut b: u32  // フォロー数
    }

    let max = 2 * a + 100;

    println!("{}", max - b)

}
