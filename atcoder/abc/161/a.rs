
use proconio::input;

fn main(){
    input!{
        mut a: i32,
        mut b: i32,
        mut c: i32,
    }
    
    let mut tmp = b;
    b = a;
    a = tmp;

    tmp = c;
    c = a;
    a = tmp;

    println!("{} {} {}", a, b, c)
}
