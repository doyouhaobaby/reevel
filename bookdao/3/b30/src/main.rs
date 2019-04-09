use std::ops::Add;

#[derive(Debug)]
struct Point {
   x: i32,
   y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("{:?}", Point {x: 1, y: 2} + Point {x: 4, y: 6});
}
