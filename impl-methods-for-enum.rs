fn main() {
    let rec = Shape::Rectangle { width: 3.0, height: 4.0 };
    let area = rec.area();
    println!("Area of rectangle is {:?} = {}", rec, area);

    let circle = Shape::Circle { radius: 1.0 };
    let area = circle.area();
    println!("Area of circle is {:?} = {}", circle, area);
}

#[derive(Debug)]
enum Shape {
    Rectangle { width: f64, height: f64},
    Circle { radius: f64 }
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Rectangle {width, height} => width * height,
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius
        }
    }
}
