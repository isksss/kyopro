use proconio::input;
use proconio::marker::Chars;

fn main(){
    input!{
        mut a: i32, // 値を受け取る
        mut b: String, // 文字列を受け取る
        mut c: [i32;a], // a個の配列を受け取る
        mut s: Chars // 文字列をVec<char>で受け取る
    }
}
