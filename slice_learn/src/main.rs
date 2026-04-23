#![allow(unused)]

use crate::slice::nth_word;
mod slice;
fn main() {
    let mut s = String::from("hello world");

    let word = slice::first_word(&s);
    println!("{word}, {s}");
    s.clear();

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}, {world}");

    let s = String::from("hello world");
    let slice = &s[0..2];
    let slice = &s[..2];
    // Rust的range语法,从索引0开始可以使用如上两种表达方式
    let s = String::from("hello world");
    let len = s.len();
    let slice = &s[3..len];
    let slic = &s[3..];
    // 可以用如上两种方法表示到最后一个字节
    let s = String::from("hello world");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];
    //可以用如上方式表示整个字符串的slice

    let s = "   Hello    World   Rust   ";
    println!("{:?}", nth_word(s, 1));
    println!("{:?}", nth_word(s, 2));
    println!("{:?}", nth_word(s, 3));
    println!("{:?}", nth_word(s, 4));
}

