mod slice;
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // 此处传入的是s1的引用,所以在下列语句中s1依旧可以使用
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("The str is : '{s}'");

    let r1 = &mut s;
    // let r2 = &mut s;    不能同时存在两个或多个变量借用同一个可变变量
    // 第二个借用的变量会把第一个覆盖掉
    // println!("{}, {}", s, r1);    同时借用的变量不可以和被借用的变量同时使用
    println!("{}", r1);
    let r2 = &mut s; //在第一个借用使用完毕后可以再次借用
    println!("{}", r2);
    // println!("{}", r1);
    /*
     *如果再次使用第一个引用变量会造成数据竞争,会出现如下报错,无法编译通过
     *For more information about this error, try `rustc --explain E0499`.
     *error: could not compile `reference_borrowing` (bin "reference_borrowing") due to 1 previous error
     */

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    // 此位置之后 r1 和 r2 不再使用,作用域结束
    let r3 = &mut s;
    println!("{r3}");

    let reference_to_noting = dangle();
}

fn calculate_length(s: &str) -> usize {
    s.len()
} // 和上个学习的例子不同
// 这个计算并返回字段长度的函数以一个对象的引用作为参数
// 而不是获取值的所有权
// s 离开了作用域,但他并不具有引用值的所有权

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}  此处本来应该返回字符串s的引用,但是当dangle这个函数结束的时候
   字符串s的内存空间被释放了,字符串s的引用就变成了一个悬垂引用
*/
fn dangle() -> String {
    let s = String::from("hello");
    s
} //解决方法,直接返回字符串
