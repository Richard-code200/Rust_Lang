fn main() {
    let x = true;
    read(x);

    let n = 5; // L1
    let y = plus_one(n); // L3
    println!("The value of y is: {y}");

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    //  println!("{full}, originally {first}");
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1 // L2
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
