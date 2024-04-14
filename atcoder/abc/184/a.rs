use proconio::input;

fn main(){
    input!{
        a: [i32; 4]
    }

    println!("{}", a[0]*a[3]-a[1]*a[2])
}
