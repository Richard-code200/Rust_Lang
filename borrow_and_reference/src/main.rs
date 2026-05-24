use std::fmt::format;

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(m1, m2);
    let s = format("{m1} {m2}!");
}

fn greet(g1: String, g2: String) {
    println!("{g1} {g2}!");
}
