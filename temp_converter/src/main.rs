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

#[cfg(test)]
mod tests {
    use super::{c_to_f, f_to_c};

    #[test]
    fn converts_freezing_point() {
        assert_eq!(c_to_f(0.0), 32.0);
        assert_eq!(f_to_c(32.0), 0.0);
    }

    #[test]
    fn converts_negative_forty() {
        assert_eq!(c_to_f(-40.0), -40.0);
        assert_eq!(f_to_c(-40.0), -40.0);
    }
}
