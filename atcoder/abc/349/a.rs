use proconio::input;

fn main(){
    input!{
        n: i64,
        a: [i64; n-1],
    };

    let sum: i64 = a.iter().sum();
    println!("{}", &sum*-1)
}
