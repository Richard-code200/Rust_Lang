// 将温度在华氏度和摄氏度之间转换
fn main() {
    println!("这是一个转换温度的程序");

    println!("{}", c_to_f(36.00));
    println!("{}", f_to_c(120.00));
}

fn c_to_f(c: f64) -> f64 {
    9f64 * c / 5f64 + 32f64
}
fn f_to_c(f: f64) -> f64 {
    5f64 * (f - 32f64) / 9f64
}
