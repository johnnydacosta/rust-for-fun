use std::ops::Add;

fn main() {
    let p1 = Point::new(2, 4);
    let p2 = Point::new(1, 1);
    let p3 = p1 + p2;
    println!("{:?} + {:?} = {:?}", p1, p2, p3);

}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y:i32) -> Point {
        Point {x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
