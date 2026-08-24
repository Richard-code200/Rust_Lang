struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let x = &mut p.x;
    print_point(&p);
    *x += 1;
}
