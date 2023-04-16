#![allow(unused, dead_code)]

#[derive(Debug)]
struct Point<A, B> {
    x: A,
    y: B,
}

impl<A, B> Point<A, B> {
    fn new(x: A, y: B) -> Self {
        Self { x, y }
    }
}

// here A,B generics are from the instance of point p calling *** self also benlongs to it. This is important
// The generics X.Y with function and its input are about input type of other , just the return x is from self hence A
impl<A, B> Point<A, B> {
    fn get_mix<X, Y>(self, other: Point<X, Y>) -> Point<A, Y> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let a = Point::new(3, 4);
    let b = Point::new("Hello", 'f');

    let c = a.get_mix(b);
    println!("c :: {:?}", &c);
}
