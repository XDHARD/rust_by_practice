#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}
fn area(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.a.y - rect.b.y).abs();
    width * height
}
fn overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = (r1.b.x.min(r2.b.x) - r1.a.x.max(r2.a.x)).max(0);
    let y_overlap = (r1.a.y.min(r2.a.y) - r1.b.y.max(r2.b.y)).max(0);
    x_overlap * y_overlap
}
fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    for i in 0..rectangles.len() {
        total_area += area(&rectangles[i]);
        for j in 0..i {
            total_area -= overlap_area(&rectangles[i], &rectangles[j]);
        }
    }
    total_area
}
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 5, y: 7 }, b: Point { x: 15, y: 5 } },
        Rectangle { a: Point { x: 12, y: 10 }, b: Point { x: 16, y: 2 } },
    ]
}
fn main() {
    let rectangles = test_data();
    let occupied_area = area_occupied(&rectangles);
    println!("Зайнята площа: {}", occupied_area);
}
