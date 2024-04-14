use proconio::input;
use proconio::marker::Chars;
use std::mem::swap; // swap(&mut x, &mut y); でxyを入れ替え
use std::f32::consts::PI; //円周率

fn main(){
    input!{
        mut a: i32, // 値を受け取る
        mut b: String, // 文字列を受け取る
        mut c: [i32;a], // a個の配列を受け取る
        mut s: Chars // 文字列をVec<char>で受け取る
    }
}
