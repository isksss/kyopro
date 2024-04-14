use proconio::input;
use std::f64::consts::PI; //円周率
fn main(){
    input!{
        r: f64, //半径
    }

    println!("{}",2.0*r*PI);
}
