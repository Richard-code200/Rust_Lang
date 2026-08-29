fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");
    greet(&m1, &m2);
    let message = format!("{m1} {m2}!");
    println!("{message}");

    // 若 greet 接收 String，m1 和 m2 会被移动，之后再使用会触发 E0382。
}

fn greet(g1: &str, g2: &str) {
    println!("{g1} {g2}!");
}
