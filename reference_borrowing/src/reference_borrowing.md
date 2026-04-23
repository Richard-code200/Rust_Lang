# 引用和借用

## 数据竞争

```Rust

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

```

该代码无法编译通过

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

这个报错说这段代码是无效的，因为我们不能在同一时间多次将 s 作为可变变量借用
第一个可变的借入在 r1 中，并且必须持续到在 println! 中使用它
但是在那个可变引用的创建和它的使用之间，我们又尝试在 r2 中创建另一个可变引用
该引用借用与 r1 相同的数据。

这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用
新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的
这个限制的好处是 Rust 可以在编译时就避免数据竞争
数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

- 两个或更多指针同时访问同一数据
- 至少有一个指针被用来写入数据
- 没有同步数据访问的机制
