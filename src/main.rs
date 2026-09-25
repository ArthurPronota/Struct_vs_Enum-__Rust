//use crate::Shape::{Circle, Rect}; // Возможно так

enum Shape {
    Circle(f64),
    Rect{w: f64, h: f64}
}

fn area(s: &Shape) ->f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect{w, h} => w * h,
    }
}

fn main() {
    let mut sh = Shape::Circle(10.) ;

    println!("Circle: {}", area(&sh));  // Circle: 314.1592653589793

    sh = Shape::Rect { w: 15., h: 20. } ;
    println!("Rect: {}", area(&sh)) ;   // Rect: 300
}
