#![allow(unused)]
fn main() {
    let mut s = String::from("hello"); // 从此处起, s 是有效的

    s.push_str(", world!"); // push_str()在字符串后追加字面值

    println!("{s}"); // 将打印'hello, world!'
    // 使用 s
    s = String::from("kayoko");
    println!("I love {s}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // 这段代码没有使用 clone，但 x 依然有效
    // 因为 i32 实现了 Copy，赋值时复制值而不是移动所有权
    // Copy 是类型语义，不能仅根据值存储在栈上还是堆上判断

    let s1 = String::from("hello");
    let s2 = s1;
    // Rust在此处做的是移动(move)而不是拷贝
    // String 的所有权从 s1 移动到 s2，之后 s1 不再有效

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");
    // 如果确实需要深度复制String中堆上的数据
    // 可以使用 clone
    // 其将复制一个完全一样的数据给新的变量

    let s5 = String::from("hello"); // s5 进入作用域

    takes_ownership(s5); // s5 的值移动到函数中
    // 所以到这里 s5 不再有效

    let z = 5; // z 进入作用域

    makes_copy(z); // i32 实现了 Copy，按值传参时复制 z 的值
    // 所以后面可以继续使用 z

    println!("{}", z);

    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);
    println!("The length of '{s7}' is {len}");

    let s1 = gives_ownership();
    // gives_ownership 将它的返回值传递给 s1

    let s2 = String::from("hello");
    // s2进入作用域

    let s3 = takes_and_gives_back(s2);
    // s2 被传入 takes_and_gives_back, 它的返回值又传递给 s3
} // 此处，s3 离开作用域并被丢弃
//  s2 被 move, 所以无事发生
//  s1 离开作用域并被丢弃
// 此作用域结束,所有变量不再有效
// 当变量离开作用域, Rust 将自动调用一个特殊的函数
// 这个函数叫 drop，它会释放值占用的资源

fn takes_ownership(some_string: String) {
    // some_string进入作用域
    println!("{some_string}");
} // 此处，some_string 离开作用域并调用 'drop' 方法
// 占用的内存被释放

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // 同上

fn gives_ownership() -> String {
    // gives_ownership 会把返回值的所有权移交给调用者
    let some_string = String::from("yours"); // some_string进入作用域
    some_string // 返回 some_string 并将其移至调用函数
}
// 该函数将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // String::len() 返回 UTF-8 内容的字节长度
    (s, length)
}
