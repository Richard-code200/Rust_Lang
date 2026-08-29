#![allow(unused)]
fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = 6;
    //let x = (let = 6);
    // 不能把 let 语句赋值给另一个变量。
    // let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值。

    let y = {
        let x = 3;
        x + 1
    }; // 这是一个代码块，其将最后的运算结果返回到 let 语句中给 y 赋值。
    // Rust 表达式结尾没有分号，表示这是返回值；加上分号后则成为语句。

    println!("The value of x is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
