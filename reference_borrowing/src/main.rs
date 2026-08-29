#![allow(unused)]
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
    // let r2 = &mut s;    r1 后续仍会使用时，不能再创建指向同一值的可变借用
    // 第二次借用不会覆盖第一次借用；若后续仍使用 r1，两个可变借用会冲突
    // println!("{}, {}", s, r1);    可变借用有效期间不能通过原变量访问该值
    println!("{}", r1);
    let r2 = &mut s; //在第一个借用使用完毕后可以再次借用
    println!("{}", r2);
    // println!("{}", r1);
    /*
     *如果在第二次可变借用后再次使用 r1，借用检查器会拒绝编译，从而避免潜在的数据竞争
     *For more information about this error, try `rustc --explain E0499`.
     *error: could not compile `reference_borrowing` (bin "reference_borrowing") due to 1 previous error
     */

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    // r1 和 r2 的共享借用在最后一次使用后基于 NLL 结束，变量的词法作用域并未结束
    let r3 = &mut s;
    println!("{r3}");

    let reference_to_nothing = dangle();
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
