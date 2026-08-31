use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    //成功:返回Ok<T>,T::std::fs::File
    //失败:返回Err<E>,E::std::io::Error
    println!("{greeting_file_result:?}");

    let v = vec![1, 2, 3];
    // v[99];
    println!("part one \n");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => panic!("Problem opening the file: {other_error:?}"),
        },
    };
    println!("part two \n");

    let greeting_file_result = File::open("hello.txt").unwrap();
    //unwrap()用于提取Option或Result类型内部的值,如果是None或Err,程序将panic并终止
    println!("{greeting_file_result:?}");

    let greeting_file_result = File::open("hello.txt").expect("File not found");
    println!("{greeting_file_result:?}");
    //expect()与unwrap()类似,但它允许自定义错误消息

    println!("\nHello, world!");
}
