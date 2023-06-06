// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line. 在前面的 match 语句中，当 y 匹配到 Some(p) 时 p 的所有权会被移动 导致 y 变得不再可用 将 y 的引用传给 match 则不会导致移动所有权
}
