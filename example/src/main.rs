struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    {
        let x = &mut p.x;
        *x += 1;
    }
    print_point(&p);

    // 若在 print_point(&p) 之后继续使用 x，会触发 E0502。
}
