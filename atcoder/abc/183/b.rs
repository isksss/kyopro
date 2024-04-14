use proconio::input;

fn main(){
    input!{
        mut sx: f64,
        mut sy: f64,
        mut gx: f64,
        mut gy: f64,
    }

    // sx sy を 0,0 に移動させる
    let nx = gx - sx;
           //6  1  2     1
    let x = (nx/(sy+gy))*sy;

    println!("{}", x+sx)
}

