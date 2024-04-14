// ---------- ---------- ----------
// 競プロRust用メモ
// > 使いそうな構文や基本的なことなどいろいろメモ
// ---------- ---------- ----------

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
    
    let num :f32 = 10.11;
    let mut x = 10;
    let mut y = 10;
    let arr: [i32; 5] = [20, 30, 50, 10, 40]; 
    let arr_def: [i32; 32] = Default::default(); // default値で埋める

    let kiriage:f32 = num.ceil(); //切り上げ
    let kirisute:f32 = num.floor(); // 切り捨て

    swap(&mut x, &mut y); // xyを入れ替え

    let arr_min = arr.iter().min(); // 最小値
    let arr_max = arr.iter().max(); // 最大値
    let min_max = arr_min.unwrap() + arr_max.unwrap(); // unwrap()しないといけない

    // String -> Vec<char>
    let mojiretu:String = "mojiretu".to_owned();
    let char_vec: Vec<char> = mojiretu.chars().collect();
    let strint_for_vec:String = char_vec.iter().collect::<String>(); // vec to string
    
}
