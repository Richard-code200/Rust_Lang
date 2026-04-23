#![allow(unused)]
fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let y = 6;
    //let x = (let = 6);
    //不能把let语句复制给另一个变量
    //let y = 6语句并不返回值,所以没有可以绑定到x上的值

    let y = {
        let x = 3;
        x + 1
    }; //这是一个代码块,其将最后的运算结果返回到let语句中给y赋值
    //rust的表达式的结尾没有分毫,表明这是最后返回的值,如果有分号则是正常语句

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
