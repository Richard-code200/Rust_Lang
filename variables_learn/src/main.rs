#![allow(unused)]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
use std::io;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let x = x + 1;
    {
        let x = x * 2; //x被遮蔽为14
        println!("The value of x in the inner scope is: {x}");
    } //内部作用域，作用域结束后内部遮蔽的作用域同时结束
    //x重新变为7
    println!("The value of x is : {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    //    let mut space = "    ";
    //    space = space.len();
    //    可以把字符串设置为可变变量，但不能改变变量的类型

    let guess: u32 = "42".parse().expect("Not a number!");
    //rust是静态类型语言，在编译的时候必须确定所有变量的类型
    //根据值及其使用方式，编译器通常可以推断处我们想要用的类型
    //当多种类型均有可能的时候，可以用转换函数
    //比如上面使用parse将String转换为u32
    //此处必须有类型注释:u32 ；否则编译器无确定需要转换的类型

    let x = 2.0;
    let y: f32 = 3.0;
    //推断类型和主动定义类型
    //rust具有常见的所有类型的变量，以及对应的bit大小

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    //常见的数值运算，加减乘除正负余

    let t = true;
    let f: bool = false;
    //逻辑判断需要的布尔值，常用于if表达式

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    //常用的字母类型，支持大多数ASCII，Unicode和特殊符号

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //元组类型，也可以不给出具体类型，编译器会自动推断
    let (x, y, z) = tup;
    //可以用匹配模式批量给变量赋值，编译器会自动匹配类型
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    //也可以用传统的方式进行索引赋值，跟大多数语言一样，第一个索引值为0

    let a = [1, 2, 3, 4, 5];
    //数组类型，依旧可以自动判断类型
    //只要不是太复杂的结构体，一般编译器都能推断出类型

    //当需要在栈(stack)而不是在堆(heap)上为数据分配空间
    //或想要确保有固定数量的元素时,数组很有用
    //如果需要更灵活的类型,可以使用vector类型
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; //数组类型一般用于这种个数和内容不太变化的变量

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //显式声明
    let a = [3; 5];
    //相当于有5个元素,且元素值都为3

    let first = a[0];
    let second = a[1];
    //依旧和传统编程语言一样使用索引赋值,但需要使用数组索引[]

    let a: [i32; 5] = [125, 222, 325, 404, 512];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    //调用std从用户获取数
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    //转换类型
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

