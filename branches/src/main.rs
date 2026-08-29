fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    //将if用于表达式赋值时两个或更多的分支中的返回值必须是相同类型的
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    } // remaining 递减，直到等于 9 时退出 loop；count 加 1，当 count 等于 2 时退出 counting_up 循环。
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    } // while 判断条件是否为 true，若为 true 则循环运行代码。
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    } //while循环通过index数量进行循环判断
    for element in a {
        println!("the value is: {element}");
    } //for循环对集合中的每一个元素进行代码执行,不用计算
    //两种循环效果相同,实现思路不同,可以细化的功能也略有不同

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
    //更加简洁的写法
}
