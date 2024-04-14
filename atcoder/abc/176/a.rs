use proconio::input;

fn main(){
    input!{
        n: f64, // n ko tukuru
        x: f64, // max
        t: f64, // 秒数
    }

    let s:f64 = (n / x).ceil();

    println!("{}", s*t)

}
