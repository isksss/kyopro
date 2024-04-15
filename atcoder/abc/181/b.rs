use proconio::input;

fn main(){
    input!{
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ans :usize = 0;

    for i in 0..n {
        let a = ab[i].0;
        let b = ab[i].1;
        for j in a..b+1{
            ans += &j;
        }
    }

    println!("{}", ans);
}
