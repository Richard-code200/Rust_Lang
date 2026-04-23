fn main() {
    let number = 3;
    if number < 5 {
        println!("condidtion was true");
    } else {
        println!("condiiton was false");
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
    } //remaining递减,知道等于9退出loop,count加1,当count等于2时退出counting_up循环
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    } //while判断条件是否为ture,若为ture则循环运行代码
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
