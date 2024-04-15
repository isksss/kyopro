use proconio::input;

fn main(){
    input!{
        a:[i64; 4],
    }
    let mut min: i64 = 100;
    for i in &a{
        if i < &min{
            min = *i;
        }
    }

    println!("{}", min);

    // これでもいいらしい
    // println!("{}", a.iter().min().unwrap());
}
